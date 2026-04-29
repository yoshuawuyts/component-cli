use rusqlite::Connection;

/// A tag pointing at a manifest within a repository.
#[derive(Debug, Clone)]
#[allow(unreachable_pub)]
pub struct OciTag {
    #[allow(dead_code)]
    id: i64,
    /// Foreign key to `oci_repository`.
    #[allow(dead_code)]
    pub oci_repository_id: i64,
    /// Digest of the manifest this tag references.
    pub manifest_digest: String,
    /// The tag string (e.g. "latest", "v1.0.0").
    #[allow(dead_code)]
    pub tag: String,
    /// When the row was created.
    #[allow(dead_code)]
    pub created_at: String,
    /// When the row was last updated.
    #[allow(dead_code)]
    pub updated_at: String,
}

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
    #[allow(dead_code)]
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
}
