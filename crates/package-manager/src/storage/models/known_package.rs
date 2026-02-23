use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, Value};

/// The type of a tag, used to distinguish release tags from signatures and attestations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TagType {
    /// A regular release tag (e.g., "1.0.0", "latest")
    Release,
    /// A signature tag (ending in ".sig")
    Signature,
    /// An attestation tag (ending in ".att")
    Attestation,
}

impl TagType {
    /// Determine the tag type from a tag string.
    pub(crate) fn from_tag(tag: &str) -> Self {
        if tag.ends_with(".sig") {
            TagType::Signature
        } else if tag.ends_with(".att") {
            TagType::Attestation
        } else {
            TagType::Release
        }
    }

    /// Convert to the database string representation.
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            TagType::Release => "release",
            TagType::Signature => "signature",
            TagType::Attestation => "attestation",
        }
    }
}

/// A known package that persists in the database even after local deletion.
/// This is used to track packages the user has seen or searched for.
#[derive(Debug, Clone)]
pub struct KnownPackage {
    #[allow(dead_code)]
    id: i64,
    /// Registry hostname
    pub registry: String,
    /// Repository path
    pub repository: String,
    /// Optional package description
    pub description: Option<String>,
    /// Release tags (regular version tags like "1.0.0", "latest")
    pub tags: Vec<String>,
    /// Signature tags (tags ending in ".sig")
    pub signature_tags: Vec<String>,
    /// Attestation tags (tags ending in ".att")
    pub attestation_tags: Vec<String>,
    /// Timestamp of last seen
    pub last_seen_at: String,
    /// Timestamp of creation
    pub created_at: String,
}

impl KnownPackage {
    /// Returns the full reference string for this package (e.g., "ghcr.io/user/repo").
    #[must_use]
    pub fn reference(&self) -> String {
        format!("{}/{}", self.registry, self.repository)
    }

    /// Returns the full reference string with the most recent tag.
    #[must_use]
    pub fn reference_with_tag(&self) -> String {
        if let Some(tag) = self.tags.first() {
            format!("{}:{}", self.reference(), tag)
        } else {
            format!("{}:latest", self.reference())
        }
    }

    /// Inserts or updates a known package in the database.
    /// If the package already exists, updates the last_seen_at timestamp.
    /// Also adds the tag if provided, classifying it by type.
    pub(crate) async fn upsert(
        conn: &DatabaseConnection,
        registry: &str,
        repository: &str,
        tag: Option<&str>,
        description: Option<&str>,
    ) -> anyhow::Result<()> {
        conn.execute(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "INSERT INTO known_package (registry, repository, description) VALUES (?, ?, ?)
             ON CONFLICT(registry, repository) DO UPDATE SET 
                last_seen_at = datetime('now'),
                description = COALESCE(excluded.description, known_package.description)",
            vec![
                Value::from(registry.to_string()),
                Value::from(repository.to_string()),
                Value::from(description.map(|s| s.to_string())),
            ],
        ))
        .await?;

        // If a tag was provided, add it to the tags table with its type
        if let Some(tag) = tag {
            let package_id: i64 = conn
                .query_one(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    "SELECT id FROM known_package WHERE registry = ? AND repository = ?",
                    vec![
                        Value::from(registry.to_string()),
                        Value::from(repository.to_string()),
                    ],
                ))
                .await?
                .expect("Package should exist after upsert")
                .try_get_by_index::<i64>(0)?;

            let tag_type = TagType::from_tag(tag);
            conn.execute(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO known_package_tag (known_package_id, tag, tag_type) VALUES (?, ?, ?)
                 ON CONFLICT(known_package_id, tag) DO UPDATE SET last_seen_at = datetime('now'), tag_type = ?",
                vec![
                    Value::from(package_id),
                    Value::from(tag.to_string()),
                    Value::from(tag_type.as_str().to_string()),
                    Value::from(tag_type.as_str().to_string()),
                ],
            ))
            .await?;
        }

        Ok(())
    }

    /// Helper to build a KnownPackage from a package model and its tags.
    fn from_package_and_tags(
        id: i64,
        registry: String,
        repository: String,
        description: Option<String>,
        last_seen_at: String,
        created_at: String,
        tags: Vec<(String, String)>,
    ) -> Self {
        let mut release_tags = Vec::new();
        let mut signature_tags = Vec::new();
        let mut attestation_tags = Vec::new();

        for (tag, tag_type) in tags {
            match tag_type.as_str() {
                "signature" => signature_tags.push(tag),
                "attestation" => attestation_tags.push(tag),
                _ => release_tags.push(tag),
            }
        }

        KnownPackage {
            id,
            registry,
            repository,
            description,
            tags: release_tags,
            signature_tags,
            attestation_tags,
            last_seen_at,
            created_at,
        }
    }

    /// Helper to fetch tags for a package by its ID, as (tag, tag_type) pairs.
    async fn fetch_tags(conn: &DatabaseConnection, package_id: i64) -> Vec<(String, String)> {
        let rows = conn
            .query_all(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT tag, tag_type FROM known_package_tag WHERE known_package_id = ? ORDER BY last_seen_at DESC",
                vec![Value::from(package_id)],
            ))
            .await
            .unwrap_or_default();

        rows.iter()
            .filter_map(|row| {
                let tag: String = row.try_get_by_index(0).ok()?;
                let tag_type: String = row.try_get_by_index(1).ok()?;
                Some((tag, tag_type))
            })
            .collect()
    }

    /// Helper to fetch packages from a query result set.
    async fn fetch_packages(
        conn: &DatabaseConnection,
        rows: Vec<sea_orm::QueryResult>,
    ) -> anyhow::Result<Vec<KnownPackage>> {
        let mut packages = Vec::new();
        for row in rows {
            let id: i64 = row.try_get_by_index(0)?;
            let registry: String = row.try_get_by_index(1)?;
            let repository: String = row.try_get_by_index(2)?;
            let description: Option<String> = row.try_get_by_index(3)?;
            let last_seen_at: String = row.try_get_by_index(4)?;
            let created_at: String = row.try_get_by_index(5)?;

            let tags = Self::fetch_tags(conn, id).await;
            packages.push(Self::from_package_and_tags(
                id,
                registry,
                repository,
                description,
                last_seen_at,
                created_at,
                tags,
            ));
        }
        Ok(packages)
    }

    /// Search for known packages by a query string.
    /// Searches in both registry and repository fields.
    pub(crate) async fn search(
        conn: &DatabaseConnection,
        query: &str,
    ) -> anyhow::Result<Vec<KnownPackage>> {
        let search_pattern = format!("%{query}%");
        let rows = conn
            .query_all(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT id, registry, repository, description, last_seen_at, created_at 
                 FROM known_package 
                 WHERE registry LIKE ? OR repository LIKE ?
                 ORDER BY repository ASC, registry ASC
                 LIMIT 100",
                vec![
                    Value::from(search_pattern.clone()),
                    Value::from(search_pattern),
                ],
            ))
            .await?;

        Self::fetch_packages(conn, rows).await
    }

    /// Get all known packages, ordered alphabetically by repository.
    pub(crate) async fn get_all(conn: &DatabaseConnection) -> anyhow::Result<Vec<KnownPackage>> {
        let rows = conn
            .query_all(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT id, registry, repository, description, last_seen_at, created_at 
                 FROM known_package 
                 ORDER BY repository ASC, registry ASC
                 LIMIT 100",
                vec![],
            ))
            .await?;

        Self::fetch_packages(conn, rows).await
    }

    /// Get a known package by registry and repository.
    #[allow(dead_code)]
    pub(crate) async fn get(
        conn: &DatabaseConnection,
        registry: &str,
        repository: &str,
    ) -> anyhow::Result<Option<KnownPackage>> {
        let row = conn
            .query_one(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT id, registry, repository, description, last_seen_at, created_at 
                 FROM known_package 
                 WHERE registry = ? AND repository = ?",
                vec![
                    Value::from(registry.to_string()),
                    Value::from(repository.to_string()),
                ],
            ))
            .await?;

        match row {
            Some(row) => {
                let id: i64 = row.try_get_by_index(0)?;
                let registry: String = row.try_get_by_index(1)?;
                let repository: String = row.try_get_by_index(2)?;
                let description: Option<String> = row.try_get_by_index(3)?;
                let last_seen_at: String = row.try_get_by_index(4)?;
                let created_at: String = row.try_get_by_index(5)?;

                let tags = Self::fetch_tags(conn, id).await;
                Ok(Some(Self::from_package_and_tags(
                    id,
                    registry,
                    repository,
                    description,
                    last_seen_at,
                    created_at,
                    tags,
                )))
            }
            None => Ok(None),
        }
    }

    /// Creates a new KnownPackage for testing purposes.
    #[cfg(any(test, feature = "test-helpers"))]
    #[must_use]
    pub fn new_for_testing(
        registry: String,
        repository: String,
        description: Option<String>,
        tags: Vec<String>,
        signature_tags: Vec<String>,
        attestation_tags: Vec<String>,
        last_seen_at: String,
        created_at: String,
    ) -> Self {
        Self {
            id: 0, // Test ID
            registry,
            repository,
            description,
            tags,
            signature_tags,
            attestation_tags,
            last_seen_at,
            created_at,
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

    // =========================================================================
    // TagType Tests
    // =========================================================================

    #[test]
    fn test_tag_type_from_tag_release() {
        assert_eq!(TagType::from_tag("latest"), TagType::Release);
        assert_eq!(TagType::from_tag("v1.0.0"), TagType::Release);
        assert_eq!(TagType::from_tag("1.2.3"), TagType::Release);
        assert_eq!(TagType::from_tag("main"), TagType::Release);
    }

    #[test]
    fn test_tag_type_from_tag_signature() {
        assert_eq!(TagType::from_tag("v1.0.0.sig"), TagType::Signature);
        assert_eq!(TagType::from_tag("latest.sig"), TagType::Signature);
        assert_eq!(TagType::from_tag(".sig"), TagType::Signature);
    }

    #[test]
    fn test_tag_type_from_tag_attestation() {
        assert_eq!(TagType::from_tag("v1.0.0.att"), TagType::Attestation);
        assert_eq!(TagType::from_tag("latest.att"), TagType::Attestation);
        assert_eq!(TagType::from_tag(".att"), TagType::Attestation);
    }

    // =========================================================================
    // KnownPackage Tests
    // =========================================================================

    #[tokio::test]
    async fn test_known_package_upsert_new_package() {
        let conn = setup_test_db().await;

        // Insert a new package
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", None, None)
            .await
            .unwrap();

        // Verify it was inserted
        let packages = KnownPackage::get_all(&conn).await.unwrap();
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].registry, "ghcr.io");
        assert_eq!(packages[0].repository, "user/repo");
    }

    #[tokio::test]
    async fn test_known_package_upsert_with_tag() {
        let conn = setup_test_db().await;

        // Insert a package with a tag
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", Some("v1.0.0"), None)
            .await
            .unwrap();

        // Verify it was inserted with the tag
        let packages = KnownPackage::get_all(&conn).await.unwrap();
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].tags, vec!["v1.0.0"]);
    }

    #[tokio::test]
    async fn test_known_package_upsert_multiple_tags() {
        let conn = setup_test_db().await;

        // Insert a package with multiple tags
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", Some("v1.0.0"), None)
            .await
            .unwrap();
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", Some("v2.0.0"), None)
            .await
            .unwrap();
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", Some("latest"), None)
            .await
            .unwrap();

        // Verify all tags are present
        let packages = KnownPackage::get_all(&conn).await.unwrap();
        assert_eq!(packages.len(), 1);
        // Tags are ordered by last_seen_at DESC
        assert!(packages[0].tags.contains(&"v1.0.0".to_string()));
        assert!(packages[0].tags.contains(&"v2.0.0".to_string()));
        assert!(packages[0].tags.contains(&"latest".to_string()));
    }

    #[tokio::test]
    async fn test_known_package_upsert_with_description() {
        let conn = setup_test_db().await;

        // Insert a package with description
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", None, Some("A test package"))
            .await
            .unwrap();

        // Verify description was saved
        let packages = KnownPackage::get_all(&conn).await.unwrap();
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].description, Some("A test package".to_string()));
    }

    #[tokio::test]
    async fn test_known_package_upsert_updates_existing() {
        let conn = setup_test_db().await;

        // Insert package
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", None, None)
            .await
            .unwrap();

        // Update with description
        KnownPackage::upsert(
            &conn,
            "ghcr.io",
            "user/repo",
            None,
            Some("Updated description"),
        )
        .await
        .unwrap();

        // Verify only one package exists with updated description
        let packages = KnownPackage::get_all(&conn).await.unwrap();
        assert_eq!(packages.len(), 1);
        assert_eq!(
            packages[0].description,
            Some("Updated description".to_string())
        );
    }

    #[tokio::test]
    async fn test_known_package_tag_types_separated() {
        let conn = setup_test_db().await;

        // Insert package with different tag types
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", Some("v1.0.0"), None)
            .await
            .unwrap();
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", Some("v1.0.0.sig"), None)
            .await
            .unwrap();
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", Some("v1.0.0.att"), None)
            .await
            .unwrap();

        // Verify tags are separated by type
        let packages = KnownPackage::get_all(&conn).await.unwrap();
        assert_eq!(packages.len(), 1);
        assert!(packages[0].tags.contains(&"v1.0.0".to_string()));
        assert!(
            packages[0]
                .signature_tags
                .contains(&"v1.0.0.sig".to_string())
        );
        assert!(
            packages[0]
                .attestation_tags
                .contains(&"v1.0.0.att".to_string())
        );
    }

    #[tokio::test]
    async fn test_known_package_search() {
        let conn = setup_test_db().await;

        // Insert multiple packages
        KnownPackage::upsert(&conn, "ghcr.io", "bytecode/component", None, None)
            .await
            .unwrap();
        KnownPackage::upsert(&conn, "docker.io", "library/nginx", None, None)
            .await
            .unwrap();
        KnownPackage::upsert(&conn, "ghcr.io", "user/nginx-app", None, None)
            .await
            .unwrap();

        // Search for nginx
        let results = KnownPackage::search(&conn, "nginx").await.unwrap();
        assert_eq!(results.len(), 2);

        // Search for ghcr.io
        let results = KnownPackage::search(&conn, "ghcr").await.unwrap();
        assert_eq!(results.len(), 2);

        // Search for bytecode
        let results = KnownPackage::search(&conn, "bytecode").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].repository, "bytecode/component");
    }

    #[tokio::test]
    async fn test_known_package_search_no_results() {
        let conn = setup_test_db().await;

        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", None, None)
            .await
            .unwrap();

        let results = KnownPackage::search(&conn, "nonexistent").await.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_known_package_get() {
        let conn = setup_test_db().await;

        // Insert a package
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", Some("v1.0.0"), None)
            .await
            .unwrap();

        // Get existing package
        let package = KnownPackage::get(&conn, "ghcr.io", "user/repo")
            .await
            .unwrap();
        assert!(package.is_some());
        let package = package.unwrap();
        assert_eq!(package.registry, "ghcr.io");
        assert_eq!(package.repository, "user/repo");

        // Get non-existent package
        let package = KnownPackage::get(&conn, "docker.io", "nonexistent")
            .await
            .unwrap();
        assert!(package.is_none());
    }

    #[tokio::test]
    async fn test_known_package_reference() {
        let conn = setup_test_db().await;

        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", None, None)
            .await
            .unwrap();

        let packages = KnownPackage::get_all(&conn).await.unwrap();
        assert_eq!(packages[0].reference(), "ghcr.io/user/repo");
    }

    #[tokio::test]
    async fn test_known_package_reference_with_tag() {
        let conn = setup_test_db().await;

        // Package without tags uses "latest"
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", None, None)
            .await
            .unwrap();
        let packages = KnownPackage::get_all(&conn).await.unwrap();
        assert_eq!(packages[0].reference_with_tag(), "ghcr.io/user/repo:latest");

        // Package with tag uses first tag
        KnownPackage::upsert(&conn, "ghcr.io", "user/repo", Some("v1.0.0"), None)
            .await
            .unwrap();
        let packages = KnownPackage::get_all(&conn).await.unwrap();
        assert_eq!(packages[0].reference_with_tag(), "ghcr.io/user/repo:v1.0.0");
    }
}
