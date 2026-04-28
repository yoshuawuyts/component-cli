use rusqlite::Connection;

/// A layer (blob) within an OCI manifest.
#[derive(Debug, Clone)]
#[allow(unreachable_pub)]
pub struct OciLayer {
    #[allow(dead_code)]
    id: i64,
    /// Foreign key to `oci_manifest`.
    #[allow(dead_code)]
    pub oci_manifest_id: i64,
    /// Content-addressable digest.
    pub digest: String,
    /// MIME type of the layer.
    #[allow(dead_code)]
    pub media_type: Option<String>,
    /// Size in bytes.
    #[allow(dead_code)]
    pub size_bytes: Option<i64>,
    /// Ordinal position within the manifest.
    #[allow(dead_code)]
    pub position: i32,
}

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
    #[allow(dead_code)]
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
}
