use oci_client::manifest::OciImageManifest;
#[cfg(any(test, feature = "test-helpers"))]
use oci_client::manifest::{IMAGE_MANIFEST_MEDIA_TYPE, OciDescriptor};
use sea_orm::DatabaseConnection;

/// Result of an insert operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertResult {
    /// The entry was inserted successfully.
    Inserted,
    /// The entry already existed in the database.
    AlreadyExists,
}

/// Metadata for a stored OCI image.
#[derive(Debug, Clone)]
pub struct ImageEntry {
    #[allow(dead_code)] // Used in database schema
    id: i64,
    /// Registry hostname
    pub ref_registry: String,
    /// Repository path
    pub ref_repository: String,
    /// Optional mirror registry hostname
    pub ref_mirror_registry: Option<String>,
    /// Optional tag
    pub ref_tag: Option<String>,
    /// Optional digest
    pub ref_digest: Option<String>,
    /// OCI image manifest
    pub manifest: OciImageManifest,
    /// Size of the image on disk in bytes
    pub size_on_disk: u64,
}

impl ImageEntry {
    /// Returns the full reference string for this image (e.g., "ghcr.io/user/repo:tag").
    #[must_use]
    pub fn reference(&self) -> String {
        let mut reference = format!("{}/{}", self.ref_registry, self.ref_repository);
        if let Some(tag) = &self.ref_tag {
            reference.push(':');
            reference.push_str(tag);
        } else if let Some(digest) = &self.ref_digest {
            reference.push('@');
            reference.push_str(digest);
        }
        reference
    }

    /// Convert a SeaORM image model to an ImageEntry.
    fn from_model(model: crate::storage::entities::image::Model) -> anyhow::Result<Self> {
        let manifest: OciImageManifest = serde_json::from_str(&model.manifest)?;
        Ok(Self {
            id: model.id,
            ref_registry: model.ref_registry,
            ref_repository: model.ref_repository,
            ref_mirror_registry: model.ref_mirror_registry,
            ref_tag: model.ref_tag,
            ref_digest: model.ref_digest,
            manifest,
            size_on_disk: model.size_on_disk as u64,
        })
    }

    /// Checks if an image entry with the given reference already exists.
    pub(crate) async fn exists(
        conn: &DatabaseConnection,
        ref_registry: &str,
        ref_repository: &str,
        ref_tag: Option<&str>,
        ref_digest: Option<&str>,
    ) -> anyhow::Result<bool> {
        use crate::storage::entities::image::{Column, Entity};
        use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};

        let mut condition = Condition::all()
            .add(Column::RefRegistry.eq(ref_registry))
            .add(Column::RefRepository.eq(ref_repository));

        match ref_tag {
            Some(tag) => condition = condition.add(Column::RefTag.eq(tag)),
            None => condition = condition.add(Column::RefTag.is_null()),
        }

        match ref_digest {
            Some(digest) => condition = condition.add(Column::RefDigest.eq(digest)),
            None => condition = condition.add(Column::RefDigest.is_null()),
        }

        let count = Entity::find().filter(condition).count(conn).await?;
        Ok(count > 0)
    }

    /// Inserts a new image entry into the database if it doesn't already exist.
    /// Returns `(InsertResult::AlreadyExists, None)` if the entry already exists,
    /// or `(InsertResult::Inserted, Some(id))` if it was successfully inserted.
    pub(crate) async fn insert(
        conn: &DatabaseConnection,
        ref_registry: &str,
        ref_repository: &str,
        ref_tag: Option<&str>,
        ref_digest: Option<&str>,
        manifest: &str,
        size_on_disk: u64,
    ) -> anyhow::Result<(InsertResult, Option<i64>)> {
        use crate::storage::entities::image;
        use sea_orm::{ActiveValue::Set, EntityTrait};

        // Check if entry already exists
        if Self::exists(conn, ref_registry, ref_repository, ref_tag, ref_digest).await? {
            return Ok((InsertResult::AlreadyExists, None));
        }

        let model = image::ActiveModel {
            ref_registry: Set(ref_registry.to_string()),
            ref_repository: Set(ref_repository.to_string()),
            ref_mirror_registry: Set(None),
            ref_tag: Set(ref_tag.map(|s| s.to_string())),
            ref_digest: Set(ref_digest.map(|s| s.to_string())),
            manifest: Set(manifest.to_string()),
            size_on_disk: Set(size_on_disk as i64),
            ..Default::default()
        };

        let result = image::Entity::insert(model).exec(conn).await?;
        Ok((InsertResult::Inserted, Some(result.last_insert_id)))
    }

    /// Returns all currently stored images and their metadata, ordered alphabetically by repository.
    pub(crate) async fn get_all(conn: &DatabaseConnection) -> anyhow::Result<Vec<ImageEntry>> {
        use crate::storage::entities::image;
        use sea_orm::{EntityTrait, QueryOrder};

        let models = image::Entity::find()
            .order_by_asc(image::Column::RefRepository)
            .order_by_asc(image::Column::RefRegistry)
            .all(conn)
            .await?;

        models.into_iter().map(Self::from_model).collect()
    }

    /// Deletes an image entry by its full reference string.
    pub(crate) async fn delete_by_reference(
        conn: &DatabaseConnection,
        registry: &str,
        repository: &str,
        tag: Option<&str>,
        digest: Option<&str>,
    ) -> anyhow::Result<bool> {
        use crate::storage::entities::image::{Column, Entity};
        use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

        let mut condition = Condition::all()
            .add(Column::RefRegistry.eq(registry))
            .add(Column::RefRepository.eq(repository));

        if let Some(tag) = tag {
            condition = condition.add(Column::RefTag.eq(tag));
        }
        if let Some(digest) = digest {
            condition = condition.add(Column::RefDigest.eq(digest));
        }

        let result = Entity::delete_many().filter(condition).exec(conn).await?;
        Ok(result.rows_affected > 0)
    }

    /// Creates a new ImageEntry for testing purposes.
    #[cfg(any(test, feature = "test-helpers"))]
    #[must_use]
    pub fn new_for_testing(
        ref_registry: String,
        ref_repository: String,
        ref_tag: Option<String>,
        ref_digest: Option<String>,
        size_on_disk: u64,
    ) -> Self {
        Self {
            id: 0,
            ref_registry,
            ref_repository,
            ref_mirror_registry: None,
            ref_tag,
            ref_digest,
            manifest: Self::test_manifest(),
            size_on_disk,
        }
    }

    /// Creates a minimal OCI image manifest with a single WASM layer for testing.
    ///
    /// The manifest uses placeholder digests and sizes that are valid but not
    /// representative of real content.
    #[cfg(any(test, feature = "test-helpers"))]
    fn test_manifest() -> OciImageManifest {
        OciImageManifest {
            schema_version: 2,
            media_type: Some(IMAGE_MANIFEST_MEDIA_TYPE.to_string()),
            config: OciDescriptor {
                media_type: "application/vnd.oci.image.config.v1+json".to_string(),
                digest: "sha256:abc123".to_string(),
                size: 100,
                urls: None,
                annotations: None,
            },
            layers: vec![OciDescriptor {
                media_type: "application/wasm".to_string(),
                digest: "sha256:def456".to_string(),
                size: 1024,
                urls: None,
                annotations: None,
            }],
            artifact_type: None,
            annotations: None,
            subject: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::models::Migrations;
    use sea_orm::Database;

    /// Create an in-memory database with migrations applied for testing.
    async fn setup_test_db() -> DatabaseConnection {
        let conn = Database::connect("sqlite::memory:").await.unwrap();
        Migrations::run_all(&conn).await.unwrap();
        conn
    }

    /// Create a minimal valid manifest JSON string for testing.
    fn test_manifest() -> String {
        r#"{"schemaVersion":2,"mediaType":"application/vnd.oci.image.manifest.v1+json","config":{"mediaType":"application/vnd.oci.image.config.v1+json","digest":"sha256:abc123","size":100},"layers":[]}"#.to_string()
    }

    // =========================================================================
    // ImageEntry Tests
    // =========================================================================

    #[tokio::test]
    async fn test_image_entry_insert_new() {
        let conn = setup_test_db().await;

        let result = ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v1.0.0"),
            None,
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();

        assert_eq!(result.0, InsertResult::Inserted);
        assert!(result.1.is_some());
    }

    #[tokio::test]
    async fn test_image_entry_insert_duplicate() {
        let conn = setup_test_db().await;

        // Insert first time
        let result1 = ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v1.0.0"),
            None,
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();
        assert_eq!(result1.0, InsertResult::Inserted);
        assert!(result1.1.is_some());

        // Insert duplicate
        let result2 = ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v1.0.0"),
            None,
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();
        assert_eq!(result2.0, InsertResult::AlreadyExists);
        assert!(result2.1.is_none());
    }

    #[tokio::test]
    async fn test_image_entry_insert_different_tags() {
        let conn = setup_test_db().await;

        // Insert with tag v1
        let result1 = ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v1.0.0"),
            None,
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();
        assert_eq!(result1.0, InsertResult::Inserted);
        assert!(result1.1.is_some());

        // Insert with tag v2 - should succeed (different tag)
        let result2 = ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v2.0.0"),
            None,
            &test_manifest(),
            2048,
        )
        .await
        .unwrap();
        assert_eq!(result2.0, InsertResult::Inserted);
        assert!(result2.1.is_some());

        // Verify both exist
        let entries = ImageEntry::get_all(&conn).await.unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[tokio::test]
    async fn test_image_entry_exists() {
        let conn = setup_test_db().await;

        // Initially doesn't exist
        assert!(
            !ImageEntry::exists(&conn, "ghcr.io", "user/repo", Some("v1.0.0"), None)
                .await
                .unwrap()
        );

        // Insert
        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v1.0.0"),
            None,
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();

        // Now exists
        assert!(
            ImageEntry::exists(&conn, "ghcr.io", "user/repo", Some("v1.0.0"), None)
                .await
                .unwrap()
        );

        // Different tag doesn't exist
        assert!(
            !ImageEntry::exists(&conn, "ghcr.io", "user/repo", Some("v2.0.0"), None)
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_image_entry_exists_with_digest() {
        let conn = setup_test_db().await;

        // Insert with digest only
        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            None,
            Some("sha256:abc123"),
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();

        // Exists with digest
        assert!(
            ImageEntry::exists(&conn, "ghcr.io", "user/repo", None, Some("sha256:abc123"))
                .await
                .unwrap()
        );

        // Different digest doesn't exist
        assert!(
            !ImageEntry::exists(&conn, "ghcr.io", "user/repo", None, Some("sha256:def456"))
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_image_entry_exists_with_tag_and_digest() {
        let conn = setup_test_db().await;

        // Insert with both tag and digest
        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v1.0.0"),
            Some("sha256:abc123"),
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();

        // Exists with both
        assert!(
            ImageEntry::exists(
                &conn,
                "ghcr.io",
                "user/repo",
                Some("v1.0.0"),
                Some("sha256:abc123")
            )
            .await
            .unwrap()
        );

        // Wrong digest doesn't match
        assert!(
            !ImageEntry::exists(
                &conn,
                "ghcr.io",
                "user/repo",
                Some("v1.0.0"),
                Some("sha256:wrong")
            )
            .await
            .unwrap()
        );
    }

    #[tokio::test]
    async fn test_image_entry_get_all_empty() {
        let conn = setup_test_db().await;

        let entries = ImageEntry::get_all(&conn).await.unwrap();
        assert!(entries.is_empty());
    }

    #[tokio::test]
    async fn test_image_entry_get_all_ordered() {
        let conn = setup_test_db().await;

        // Insert in non-alphabetical order
        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "zebra/repo",
            Some("v1.0.0"),
            None,
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();
        ImageEntry::insert(
            &conn,
            "docker.io",
            "apple/repo",
            Some("latest"),
            None,
            &test_manifest(),
            2048,
        )
        .await
        .unwrap();

        let entries = ImageEntry::get_all(&conn).await.unwrap();
        assert_eq!(entries.len(), 2);
        // Should be ordered by repository ASC
        assert_eq!(entries[0].ref_repository, "apple/repo");
        assert_eq!(entries[1].ref_repository, "zebra/repo");
    }

    #[tokio::test]
    async fn test_image_entry_delete_by_reference_with_tag() {
        let conn = setup_test_db().await;

        // Insert
        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v1.0.0"),
            None,
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();

        // Delete
        let deleted =
            ImageEntry::delete_by_reference(&conn, "ghcr.io", "user/repo", Some("v1.0.0"), None)
                .await
                .unwrap();
        assert!(deleted);

        // Verify gone
        assert!(ImageEntry::get_all(&conn).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_image_entry_delete_by_reference_not_found() {
        let conn = setup_test_db().await;

        // Try to delete non-existent
        let deleted = ImageEntry::delete_by_reference(
            &conn,
            "ghcr.io",
            "nonexistent/repo",
            Some("v1.0.0"),
            None,
        )
        .await
        .unwrap();
        assert!(!deleted);
    }

    #[tokio::test]
    async fn test_image_entry_delete_by_reference_with_digest() {
        let conn = setup_test_db().await;

        // Insert with digest
        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            None,
            Some("sha256:abc123"),
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();

        // Delete by digest
        let deleted = ImageEntry::delete_by_reference(
            &conn,
            "ghcr.io",
            "user/repo",
            None,
            Some("sha256:abc123"),
        )
        .await
        .unwrap();
        assert!(deleted);

        assert!(ImageEntry::get_all(&conn).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_image_entry_delete_by_registry_repository_only() {
        let conn = setup_test_db().await;

        // Insert multiple entries for same repo
        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v1.0.0"),
            None,
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();
        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v2.0.0"),
            None,
            &test_manifest(),
            2048,
        )
        .await
        .unwrap();

        // Delete all by registry/repository only
        let deleted = ImageEntry::delete_by_reference(&conn, "ghcr.io", "user/repo", None, None)
            .await
            .unwrap();
        assert!(deleted);

        assert!(ImageEntry::get_all(&conn).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_image_entry_reference_with_tag() {
        let conn = setup_test_db().await;

        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v1.0.0"),
            None,
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();

        let entries = ImageEntry::get_all(&conn).await.unwrap();
        assert_eq!(entries[0].reference(), "ghcr.io/user/repo:v1.0.0");
    }

    #[tokio::test]
    async fn test_image_entry_reference_with_digest() {
        let conn = setup_test_db().await;

        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            None,
            Some("sha256:abc123"),
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();

        let entries = ImageEntry::get_all(&conn).await.unwrap();
        assert_eq!(entries[0].reference(), "ghcr.io/user/repo@sha256:abc123");
    }

    #[tokio::test]
    async fn test_image_entry_reference_plain() {
        let conn = setup_test_db().await;

        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            None,
            None,
            &test_manifest(),
            1024,
        )
        .await
        .unwrap();

        let entries = ImageEntry::get_all(&conn).await.unwrap();
        assert_eq!(entries[0].reference(), "ghcr.io/user/repo");
    }

    #[tokio::test]
    async fn test_image_entry_size_on_disk() {
        let conn = setup_test_db().await;

        ImageEntry::insert(
            &conn,
            "ghcr.io",
            "user/repo",
            Some("v1.0.0"),
            None,
            &test_manifest(),
            12345678,
        )
        .await
        .unwrap();

        let entries = ImageEntry::get_all(&conn).await.unwrap();
        assert_eq!(entries[0].size_on_disk, 12345678);
    }
}
