use rusqlite::Connection;

/// A referrer relationship between two manifests.
#[derive(Debug, Clone)]
#[allow(unreachable_pub)]
pub struct OciReferrer {
    #[allow(dead_code)]
    id: i64,
    /// The manifest that is being referred to.
    #[allow(dead_code)]
    pub subject_manifest_id: i64,
    /// The manifest doing the referring.
    #[allow(dead_code)]
    pub referrer_manifest_id: i64,
    /// The artifact type of the referrer.
    #[allow(dead_code)]
    pub artifact_type: String,
    /// When the row was created.
    #[allow(dead_code)]
    pub created_at: String,
}

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
    #[allow(dead_code)]
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
}
