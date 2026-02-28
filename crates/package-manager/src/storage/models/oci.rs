use std::collections::HashMap;

use rusqlite::Connection;

/// Well-known OCI annotation keys that are extracted into dedicated columns.
const WELL_KNOWN_ANNOTATIONS: &[(&str, &str)] = &[
    ("org.opencontainers.image.created", "oci_created"),
    ("org.opencontainers.image.authors", "oci_authors"),
    ("org.opencontainers.image.url", "oci_url"),
    (
        "org.opencontainers.image.documentation",
        "oci_documentation",
    ),
    ("org.opencontainers.image.source", "oci_source"),
    ("org.opencontainers.image.version", "oci_version"),
    ("org.opencontainers.image.revision", "oci_revision"),
    ("org.opencontainers.image.vendor", "oci_vendor"),
    ("org.opencontainers.image.licenses", "oci_licenses"),
    ("org.opencontainers.image.ref.name", "oci_ref_name"),
    ("org.opencontainers.image.title", "oci_title"),
    ("org.opencontainers.image.description", "oci_description"),
    ("org.opencontainers.image.base.digest", "oci_base_digest"),
    ("org.opencontainers.image.base.name", "oci_base_name"),
];

/// Result of an insert operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code, unreachable_pub)]
pub enum InsertResult {
    /// The entry was inserted successfully.
    Inserted,
    /// The entry already existed in the database.
    AlreadyExists,
}

// ---------------------------------------------------------------------------
// OciRepository
// ---------------------------------------------------------------------------

/// An OCI registry/repository pair.
#[derive(Debug, Clone)]
#[allow(dead_code, unreachable_pub)]
pub struct OciRepository {
    #[allow(dead_code)]
    id: i64,
    /// Registry hostname (e.g. "ghcr.io").
    pub registry: String,
    /// Repository path (e.g. "user/repo").
    pub repository: String,
    /// When the row was created.
    pub created_at: String,
    /// When the row was last updated.
    pub updated_at: String,
}

#[allow(dead_code)]
impl OciRepository {
    /// Returns the primary key.
    #[must_use]
    pub(crate) fn id(&self) -> i64 {
        self.id
    }

    /// Insert or update a repository, returning its row id.
    pub(crate) fn upsert(
        conn: &Connection,
        registry: &str,
        repository: &str,
    ) -> anyhow::Result<i64> {
        conn.execute(
            "INSERT INTO oci_repository (registry, repository)
             VALUES (?1, ?2)
             ON CONFLICT(registry, repository) DO UPDATE SET
                 updated_at = CURRENT_TIMESTAMP",
            (registry, repository),
        )?;

        let id: i64 = conn.query_row(
            "SELECT id FROM oci_repository WHERE registry = ?1 AND repository = ?2",
            (registry, repository),
            |row| row.get(0),
        )?;

        Ok(id)
    }

    /// Get a repository by its primary key.
    pub(crate) fn get_by_id(conn: &Connection, id: i64) -> anyhow::Result<Option<Self>> {
        let result = conn.query_row(
            "SELECT id, registry, repository, created_at, updated_at
             FROM oci_repository WHERE id = ?1",
            [id],
            |row| {
                Ok(Self {
                    id: row.get(0)?,
                    registry: row.get(1)?,
                    repository: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        );

        match result {
            Ok(repo) => Ok(Some(repo)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Find a repository by registry and repository name.
    pub(crate) fn find(
        conn: &Connection,
        registry: &str,
        repository: &str,
    ) -> anyhow::Result<Option<Self>> {
        let result = conn.query_row(
            "SELECT id, registry, repository, created_at, updated_at
             FROM oci_repository WHERE registry = ?1 AND repository = ?2",
            (registry, repository),
            |row| {
                Ok(Self {
                    id: row.get(0)?,
                    registry: row.get(1)?,
                    repository: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        );

        match result {
            Ok(repo) => Ok(Some(repo)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// List every repository.
    pub(crate) fn list_all(conn: &Connection) -> anyhow::Result<Vec<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, registry, repository, created_at, updated_at
             FROM oci_repository ORDER BY repository ASC, registry ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(Self {
                id: row.get(0)?,
                registry: row.get(1)?,
                repository: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Creates a new `OciRepository` for testing purposes.
    #[cfg(any(test, feature = "test-helpers"))]
    #[must_use]
    pub fn new_for_testing(
        registry: String,
        repository: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id: 0,
            registry,
            repository,
            created_at,
            updated_at,
        }
    }
}

// ---------------------------------------------------------------------------
// OciManifest
// ---------------------------------------------------------------------------

/// An OCI image manifest stored in the database.
#[derive(Debug, Clone)]
#[allow(dead_code, unreachable_pub)]
pub struct OciManifest {
    #[allow(dead_code)]
    id: i64,
    /// Foreign key to `oci_repository`.
    pub oci_repository_id: i64,
    /// Content-addressable digest.
    pub digest: String,
    /// MIME type of the manifest.
    pub media_type: Option<String>,
    /// Raw JSON body.
    pub raw_json: Option<String>,
    /// Total size in bytes.
    pub size_bytes: Option<i64>,
    /// When the row was created.
    pub created_at: String,
    /// Artifact type, if present.
    pub artifact_type: Option<String>,
    /// Config descriptor media type.
    pub config_media_type: Option<String>,
    /// Config descriptor digest.
    pub config_digest: Option<String>,
    /// `org.opencontainers.image.created`
    pub oci_created: Option<String>,
    /// `org.opencontainers.image.authors`
    pub oci_authors: Option<String>,
    /// `org.opencontainers.image.url`
    pub oci_url: Option<String>,
    /// `org.opencontainers.image.documentation`
    pub oci_documentation: Option<String>,
    /// `org.opencontainers.image.source`
    pub oci_source: Option<String>,
    /// `org.opencontainers.image.version`
    pub oci_version: Option<String>,
    /// `org.opencontainers.image.revision`
    pub oci_revision: Option<String>,
    /// `org.opencontainers.image.vendor`
    pub oci_vendor: Option<String>,
    /// `org.opencontainers.image.licenses`
    pub oci_licenses: Option<String>,
    /// `org.opencontainers.image.ref.name`
    pub oci_ref_name: Option<String>,
    /// `org.opencontainers.image.title`
    pub oci_title: Option<String>,
    /// `org.opencontainers.image.description`
    pub oci_description: Option<String>,
    /// `org.opencontainers.image.base.digest`
    pub oci_base_digest: Option<String>,
    /// `org.opencontainers.image.base.name`
    pub oci_base_name: Option<String>,
}

#[allow(dead_code)]
impl OciManifest {
    /// Returns the primary key.
    #[must_use]
    #[allow(dead_code, unreachable_pub)]
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Insert a manifest and its annotations.
    ///
    /// Well-known OCI annotation keys are extracted into dedicated columns;
    /// remaining annotations are stored in `oci_manifest_annotation`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn insert(
        conn: &Connection,
        oci_repository_id: i64,
        digest: &str,
        media_type: Option<&str>,
        raw_json: Option<&str>,
        size_bytes: Option<i64>,
        annotations: &HashMap<String, String>,
    ) -> anyhow::Result<i64> {
        // Partition annotations into well-known and extra.
        let ann_key_to_col: HashMap<&str, &str> = WELL_KNOWN_ANNOTATIONS.iter().copied().collect();
        let mut well_known: HashMap<&str, &str> = HashMap::new();
        let mut extra: Vec<(&str, &str)> = Vec::new();

        for (k, v) in annotations {
            if let Some(&col) = ann_key_to_col.get(k.as_str()) {
                well_known.insert(col, v.as_str());
            } else {
                extra.push((k.as_str(), v.as_str()));
            }
        }

        conn.execute(
            "INSERT INTO oci_manifest (
                oci_repository_id, digest, media_type, raw_json, size_bytes,
                oci_created, oci_authors, oci_url, oci_documentation, oci_source,
                oci_version, oci_revision, oci_vendor, oci_licenses, oci_ref_name,
                oci_title, oci_description, oci_base_digest, oci_base_name
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5,
                ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15,
                ?16, ?17, ?18, ?19
             )",
            rusqlite::params![
                oci_repository_id,
                digest,
                media_type,
                raw_json,
                size_bytes,
                well_known.get("oci_created").copied(),
                well_known.get("oci_authors").copied(),
                well_known.get("oci_url").copied(),
                well_known.get("oci_documentation").copied(),
                well_known.get("oci_source").copied(),
                well_known.get("oci_version").copied(),
                well_known.get("oci_revision").copied(),
                well_known.get("oci_vendor").copied(),
                well_known.get("oci_licenses").copied(),
                well_known.get("oci_ref_name").copied(),
                well_known.get("oci_title").copied(),
                well_known.get("oci_description").copied(),
                well_known.get("oci_base_digest").copied(),
                well_known.get("oci_base_name").copied(),
            ],
        )?;

        let manifest_id = conn.last_insert_rowid();

        // Store extra (non-well-known) annotations.
        for (key, value) in &extra {
            conn.execute(
                "INSERT INTO oci_manifest_annotation (oci_manifest_id, `key`, `value`)
                 VALUES (?1, ?2, ?3)
                 ON CONFLICT(oci_manifest_id, `key`) DO UPDATE SET `value` = ?3",
                rusqlite::params![manifest_id, key, value],
            )?;
        }

        Ok(manifest_id)
    }

    /// Get a manifest by primary key.
    pub(crate) fn get_by_id(conn: &Connection, id: i64) -> anyhow::Result<Option<Self>> {
        let result = conn.query_row(
            "SELECT id, oci_repository_id, digest, media_type, raw_json, size_bytes,
                    created_at, artifact_type, config_media_type, config_digest,
                    oci_created, oci_authors, oci_url, oci_documentation, oci_source,
                    oci_version, oci_revision, oci_vendor, oci_licenses, oci_ref_name,
                    oci_title, oci_description, oci_base_digest, oci_base_name
             FROM oci_manifest WHERE id = ?1",
            [id],
            Self::from_row,
        );

        match result {
            Ok(m) => Ok(Some(m)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Find a manifest by repository id and digest.
    pub(crate) fn find(
        conn: &Connection,
        oci_repository_id: i64,
        digest: &str,
    ) -> anyhow::Result<Option<Self>> {
        let result = conn.query_row(
            "SELECT id, oci_repository_id, digest, media_type, raw_json, size_bytes,
                    created_at, artifact_type, config_media_type, config_digest,
                    oci_created, oci_authors, oci_url, oci_documentation, oci_source,
                    oci_version, oci_revision, oci_vendor, oci_licenses, oci_ref_name,
                    oci_title, oci_description, oci_base_digest, oci_base_name
             FROM oci_manifest WHERE oci_repository_id = ?1 AND digest = ?2",
            (oci_repository_id, digest),
            Self::from_row,
        );

        match result {
            Ok(m) => Ok(Some(m)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// List all manifests belonging to a repository.
    pub(crate) fn list_by_repository(
        conn: &Connection,
        oci_repository_id: i64,
    ) -> anyhow::Result<Vec<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, oci_repository_id, digest, media_type, raw_json, size_bytes,
                    created_at, artifact_type, config_media_type, config_digest,
                    oci_created, oci_authors, oci_url, oci_documentation, oci_source,
                    oci_version, oci_revision, oci_vendor, oci_licenses, oci_ref_name,
                    oci_title, oci_description, oci_base_digest, oci_base_name
             FROM oci_manifest WHERE oci_repository_id = ?1 ORDER BY created_at ASC",
        )?;

        let rows = stmt.query_map([oci_repository_id], Self::from_row)?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Delete a manifest by primary key. Returns `true` if a row was removed.
    pub(crate) fn delete(conn: &Connection, id: i64) -> anyhow::Result<bool> {
        let rows = conn.execute("DELETE FROM oci_manifest WHERE id = ?1", [id])?;
        Ok(rows > 0)
    }

    /// Map a `rusqlite::Row` to `Self`.
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            oci_repository_id: row.get(1)?,
            digest: row.get(2)?,
            media_type: row.get(3)?,
            raw_json: row.get(4)?,
            size_bytes: row.get(5)?,
            created_at: row.get(6)?,
            artifact_type: row.get(7)?,
            config_media_type: row.get(8)?,
            config_digest: row.get(9)?,
            oci_created: row.get(10)?,
            oci_authors: row.get(11)?,
            oci_url: row.get(12)?,
            oci_documentation: row.get(13)?,
            oci_source: row.get(14)?,
            oci_version: row.get(15)?,
            oci_revision: row.get(16)?,
            oci_vendor: row.get(17)?,
            oci_licenses: row.get(18)?,
            oci_ref_name: row.get(19)?,
            oci_title: row.get(20)?,
            oci_description: row.get(21)?,
            oci_base_digest: row.get(22)?,
            oci_base_name: row.get(23)?,
        })
    }

    /// Creates a new `OciManifest` for testing purposes.
    #[cfg(any(test, feature = "test-helpers"))]
    #[must_use]
    pub fn new_for_testing(oci_repository_id: i64, digest: String) -> Self {
        Self {
            id: 0,
            oci_repository_id,
            digest,
            media_type: None,
            raw_json: None,
            size_bytes: None,
            created_at: String::new(),
            artifact_type: None,
            config_media_type: None,
            config_digest: None,
            oci_created: None,
            oci_authors: None,
            oci_url: None,
            oci_documentation: None,
            oci_source: None,
            oci_version: None,
            oci_revision: None,
            oci_vendor: None,
            oci_licenses: None,
            oci_ref_name: None,
            oci_title: None,
            oci_description: None,
            oci_base_digest: None,
            oci_base_name: None,
        }
    }
}

// ---------------------------------------------------------------------------
// OciTag
// ---------------------------------------------------------------------------

/// A tag pointing at a manifest within a repository.
#[derive(Debug, Clone)]
#[allow(dead_code, unreachable_pub)]
pub struct OciTag {
    #[allow(dead_code)]
    id: i64,
    /// Foreign key to `oci_repository`.
    pub oci_repository_id: i64,
    /// Digest of the manifest this tag references.
    pub manifest_digest: String,
    /// The tag string (e.g. "latest", "v1.0.0").
    pub tag: String,
    /// When the row was created.
    pub created_at: String,
    /// When the row was last updated.
    pub updated_at: String,
}

#[allow(dead_code)]
impl OciTag {
    /// Insert or update a tag, returning its row id.
    pub(crate) fn upsert(
        conn: &Connection,
        oci_repository_id: i64,
        tag: &str,
        manifest_digest: &str,
    ) -> anyhow::Result<i64> {
        conn.execute(
            "INSERT INTO oci_tag (oci_repository_id, tag, manifest_digest)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(oci_repository_id, tag) DO UPDATE SET
                 manifest_digest = ?3,
                 updated_at = CURRENT_TIMESTAMP",
            (oci_repository_id, tag, manifest_digest),
        )?;

        let id: i64 = conn.query_row(
            "SELECT id FROM oci_tag WHERE oci_repository_id = ?1 AND tag = ?2",
            (oci_repository_id, tag),
            |row| row.get(0),
        )?;

        Ok(id)
    }

    /// List all tags for a repository.
    pub(crate) fn list_by_repository(
        conn: &Connection,
        oci_repository_id: i64,
    ) -> anyhow::Result<Vec<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, oci_repository_id, manifest_digest, tag, created_at, updated_at
             FROM oci_tag WHERE oci_repository_id = ?1 ORDER BY tag ASC",
        )?;

        let rows = stmt.query_map([oci_repository_id], |row| {
            Ok(Self {
                id: row.get(0)?,
                oci_repository_id: row.get(1)?,
                manifest_digest: row.get(2)?,
                tag: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Find a tag by repository id and tag name.
    pub(crate) fn find_by_tag(
        conn: &Connection,
        oci_repository_id: i64,
        tag: &str,
    ) -> anyhow::Result<Option<Self>> {
        let result = conn.query_row(
            "SELECT id, oci_repository_id, manifest_digest, tag, created_at, updated_at
             FROM oci_tag WHERE oci_repository_id = ?1 AND tag = ?2",
            (oci_repository_id, tag),
            |row| {
                Ok(Self {
                    id: row.get(0)?,
                    oci_repository_id: row.get(1)?,
                    manifest_digest: row.get(2)?,
                    tag: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        );

        match result {
            Ok(t) => Ok(Some(t)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Creates a new `OciTag` for testing purposes.
    #[cfg(any(test, feature = "test-helpers"))]
    #[must_use]
    pub fn new_for_testing(oci_repository_id: i64, tag: String, manifest_digest: String) -> Self {
        Self {
            id: 0,
            oci_repository_id,
            manifest_digest,
            tag,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// OciLayer
// ---------------------------------------------------------------------------

/// A layer (blob) within an OCI manifest.
#[derive(Debug, Clone)]
#[allow(dead_code, unreachable_pub)]
pub struct OciLayer {
    #[allow(dead_code)]
    id: i64,
    /// Foreign key to `oci_manifest`.
    pub oci_manifest_id: i64,
    /// Content-addressable digest.
    pub digest: String,
    /// MIME type of the layer.
    pub media_type: Option<String>,
    /// Size in bytes.
    pub size_bytes: Option<i64>,
    /// Ordinal position within the manifest.
    pub position: i32,
}

#[allow(dead_code)]
impl OciLayer {
    /// Insert a new layer, returning its row id.
    pub(crate) fn insert(
        conn: &Connection,
        oci_manifest_id: i64,
        digest: &str,
        media_type: Option<&str>,
        size_bytes: Option<i64>,
        position: i32,
    ) -> anyhow::Result<i64> {
        conn.execute(
            "INSERT INTO oci_layer (oci_manifest_id, digest, media_type, size_bytes, position)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![oci_manifest_id, digest, media_type, size_bytes, position],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// List all layers for a manifest, ordered by position.
    pub(crate) fn list_by_manifest(
        conn: &Connection,
        oci_manifest_id: i64,
    ) -> anyhow::Result<Vec<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, oci_manifest_id, digest, media_type, size_bytes, position
             FROM oci_layer WHERE oci_manifest_id = ?1 ORDER BY position ASC",
        )?;

        let rows = stmt.query_map([oci_manifest_id], |row| {
            Ok(Self {
                id: row.get(0)?,
                oci_manifest_id: row.get(1)?,
                digest: row.get(2)?,
                media_type: row.get(3)?,
                size_bytes: row.get(4)?,
                position: row.get(5)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Find a layer by manifest id and digest.
    pub(crate) fn get_by_digest(
        conn: &Connection,
        oci_manifest_id: i64,
        digest: &str,
    ) -> anyhow::Result<Option<Self>> {
        let result = conn.query_row(
            "SELECT id, oci_manifest_id, digest, media_type, size_bytes, position
             FROM oci_layer WHERE oci_manifest_id = ?1 AND digest = ?2",
            (oci_manifest_id, digest),
            |row| {
                Ok(Self {
                    id: row.get(0)?,
                    oci_manifest_id: row.get(1)?,
                    digest: row.get(2)?,
                    media_type: row.get(3)?,
                    size_bytes: row.get(4)?,
                    position: row.get(5)?,
                })
            },
        );

        match result {
            Ok(l) => Ok(Some(l)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Creates a new `OciLayer` for testing purposes.
    #[cfg(any(test, feature = "test-helpers"))]
    #[must_use]
    pub fn new_for_testing(
        oci_manifest_id: i64,
        digest: String,
        media_type: Option<String>,
        size_bytes: Option<i64>,
        position: i32,
    ) -> Self {
        Self {
            id: 0,
            oci_manifest_id,
            digest,
            media_type,
            size_bytes,
            position,
        }
    }
}

// ---------------------------------------------------------------------------
// OciReferrer
// ---------------------------------------------------------------------------

/// A referrer relationship between two manifests.
#[derive(Debug, Clone)]
#[allow(dead_code, unreachable_pub)]
pub struct OciReferrer {
    #[allow(dead_code)]
    id: i64,
    /// The manifest that is being referred to.
    pub subject_manifest_id: i64,
    /// The manifest doing the referring.
    pub referrer_manifest_id: i64,
    /// The artifact type of the referrer.
    pub artifact_type: String,
    /// When the row was created.
    pub created_at: String,
}

#[allow(dead_code)]
impl OciReferrer {
    /// Insert a new referrer relationship, returning its row id.
    pub(crate) fn insert(
        conn: &Connection,
        subject_manifest_id: i64,
        referrer_manifest_id: i64,
        artifact_type: &str,
    ) -> anyhow::Result<i64> {
        conn.execute(
            "INSERT INTO oci_referrer (subject_manifest_id, referrer_manifest_id, artifact_type)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(subject_manifest_id, referrer_manifest_id) DO NOTHING",
            (subject_manifest_id, referrer_manifest_id, artifact_type),
        )?;

        let id: i64 = conn.query_row(
            "SELECT id FROM oci_referrer
             WHERE subject_manifest_id = ?1 AND referrer_manifest_id = ?2",
            (subject_manifest_id, referrer_manifest_id),
            |row| row.get(0),
        )?;

        Ok(id)
    }

    /// List all referrers for a given subject manifest.
    pub(crate) fn list_by_subject(
        conn: &Connection,
        subject_manifest_id: i64,
    ) -> anyhow::Result<Vec<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, subject_manifest_id, referrer_manifest_id, artifact_type, created_at
             FROM oci_referrer WHERE subject_manifest_id = ?1 ORDER BY created_at ASC",
        )?;

        let rows = stmt.query_map([subject_manifest_id], |row| {
            Ok(Self {
                id: row.get(0)?,
                subject_manifest_id: row.get(1)?,
                referrer_manifest_id: row.get(2)?,
                artifact_type: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Creates a new `OciReferrer` for testing purposes.
    #[cfg(any(test, feature = "test-helpers"))]
    #[must_use]
    pub fn new_for_testing(
        subject_manifest_id: i64,
        referrer_manifest_id: i64,
        artifact_type: String,
    ) -> Self {
        Self {
            id: 0,
            subject_manifest_id,
            referrer_manifest_id,
            artifact_type,
            created_at: String::new(),
        }
    }
}
