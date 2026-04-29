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

/// An OCI image manifest stored in the database.
///
/// Many annotation fields are not yet consumed by the current code paths
/// but are populated from OCI manifests for completeness.
#[derive(Debug, Clone)]
#[allow(dead_code, unreachable_pub)]
pub struct OciManifest {
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

impl OciManifest {
    /// Returns the primary key.
    #[must_use]
    #[allow(unreachable_pub)]
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Insert a manifest and its annotations.
    ///
    /// Well-known OCI annotation keys are extracted into dedicated columns;
    /// remaining annotations are stored in `oci_manifest_annotation`.
    ///
    /// Uses `INSERT … ON CONFLICT DO UPDATE SET` with `COALESCE` so that
    /// placeholder rows (e.g. from referrer discovery) are filled in when
    /// a full pull supplies non-NULL data. Returns `(manifest_id, was_inserted)`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn upsert(
        conn: &Connection,
        oci_repository_id: i64,
        digest: &str,
        media_type: Option<&str>,
        raw_json: Option<&str>,
        size_bytes: Option<i64>,
        artifact_type: Option<&str>,
        config_media_type: Option<&str>,
        config_digest: Option<&str>,
        annotations: &HashMap<String, String>,
    ) -> anyhow::Result<(i64, bool)> {
        // Partition annotations into well-known and extra.
        let ann_key_to_col: HashMap<&str, &str> = WELL_KNOWN_ANNOTATIONS.iter().copied().collect();
        let mut well_known: HashMap<&str, &str> = HashMap::new();
        let mut extra: Vec<(&str, &str)> = Vec::new();

        for (k, v) in annotations {
            match ann_key_to_col.get(k.as_str()) {
                Some(&col) => {
                    well_known.insert(col, v.as_str());
                }
                None => extra.push((k.as_str(), v.as_str())),
            }
        }

        // Check if the row already exists so we can report was_inserted correctly.
        let already_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM oci_manifest
                 WHERE oci_repository_id = ?1 AND digest = ?2",
                (oci_repository_id, digest),
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0)
            > 0;

        // COALESCE keeps the existing non-NULL value, only filling in NULLs
        // from the incoming data. This lets placeholder manifests (created by
        // referrer discovery) be upgraded later by a full pull.
        conn.execute(
            "INSERT INTO oci_manifest (
                oci_repository_id, digest, media_type, raw_json, size_bytes,
                artifact_type, config_media_type, config_digest,
                oci_created, oci_authors, oci_url, oci_documentation, oci_source,
                oci_version, oci_revision, oci_vendor, oci_licenses, oci_ref_name,
                oci_title, oci_description, oci_base_digest, oci_base_name
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5,
                ?6, ?7, ?8,
                ?9, ?10, ?11, ?12, ?13,
                ?14, ?15, ?16, ?17, ?18,
                ?19, ?20, ?21, ?22
             )
             ON CONFLICT(oci_repository_id, digest) DO UPDATE SET
                media_type       = COALESCE(excluded.media_type,       oci_manifest.media_type),
                raw_json         = COALESCE(excluded.raw_json,         oci_manifest.raw_json),
                size_bytes       = COALESCE(excluded.size_bytes,       oci_manifest.size_bytes),
                artifact_type    = COALESCE(excluded.artifact_type,    oci_manifest.artifact_type),
                config_media_type= COALESCE(excluded.config_media_type,oci_manifest.config_media_type),
                config_digest    = COALESCE(excluded.config_digest,    oci_manifest.config_digest),
                oci_created      = COALESCE(excluded.oci_created,      oci_manifest.oci_created),
                oci_authors      = COALESCE(excluded.oci_authors,      oci_manifest.oci_authors),
                oci_url          = COALESCE(excluded.oci_url,          oci_manifest.oci_url),
                oci_documentation= COALESCE(excluded.oci_documentation,oci_manifest.oci_documentation),
                oci_source       = COALESCE(excluded.oci_source,       oci_manifest.oci_source),
                oci_version      = COALESCE(excluded.oci_version,      oci_manifest.oci_version),
                oci_revision     = COALESCE(excluded.oci_revision,     oci_manifest.oci_revision),
                oci_vendor       = COALESCE(excluded.oci_vendor,       oci_manifest.oci_vendor),
                oci_licenses     = COALESCE(excluded.oci_licenses,     oci_manifest.oci_licenses),
                oci_ref_name     = COALESCE(excluded.oci_ref_name,     oci_manifest.oci_ref_name),
                oci_title        = COALESCE(excluded.oci_title,        oci_manifest.oci_title),
                oci_description  = COALESCE(excluded.oci_description,  oci_manifest.oci_description),
                oci_base_digest  = COALESCE(excluded.oci_base_digest,  oci_manifest.oci_base_digest),
                oci_base_name    = COALESCE(excluded.oci_base_name,    oci_manifest.oci_base_name)",
            rusqlite::params![
                oci_repository_id,
                digest,
                media_type,
                raw_json,
                size_bytes,
                artifact_type,
                config_media_type,
                config_digest,
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

        let was_inserted = !already_exists;

        // Retrieve the canonical row id.
        let manifest_id: i64 = conn.query_row(
            "SELECT id FROM oci_manifest WHERE oci_repository_id = ?1 AND digest = ?2",
            (oci_repository_id, digest),
            |row| row.get(0),
        )?;

        // Store extra (non-well-known) annotations.
        for (key, value) in &extra {
            conn.execute(
                "INSERT INTO oci_manifest_annotation (oci_manifest_id, `key`, `value`)
                 VALUES (?1, ?2, ?3)
                 ON CONFLICT(oci_manifest_id, `key`) DO UPDATE SET `value` = ?3",
                rusqlite::params![manifest_id, key, value],
            )?;
        }

        Ok((manifest_id, was_inserted))
    }

    /// Get a manifest by primary key.
    #[allow(dead_code)]
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
}
