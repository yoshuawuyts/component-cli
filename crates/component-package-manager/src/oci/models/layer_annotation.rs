use rusqlite::Connection;

/// A key-value annotation on an OCI layer.
///
/// Some Wasm toolchains attach metadata at the layer descriptor level rather
/// than the manifest level. This table captures those annotations.
#[derive(Debug, Clone)]
#[allow(unreachable_pub)]
pub struct OciLayerAnnotation {
    #[allow(dead_code)]
    id: i64,
    /// Foreign key to `oci_layer`.
    #[allow(dead_code)]
    pub oci_layer_id: i64,
    /// The full annotation key.
    pub key: String,
    /// The annotation value.
    pub value: String,
}

impl OciLayerAnnotation {
    /// Insert a layer annotation, upserting on conflict.
    pub(crate) fn insert(
        conn: &Connection,
        oci_layer_id: i64,
        key: &str,
        value: &str,
    ) -> anyhow::Result<i64> {
        conn.execute(
            "INSERT INTO oci_layer_annotation (oci_layer_id, `key`, `value`)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(oci_layer_id, `key`) DO UPDATE SET `value` = ?3",
            rusqlite::params![oci_layer_id, key, value],
        )?;

        let id: i64 = conn.query_row(
            "SELECT id FROM oci_layer_annotation
             WHERE oci_layer_id = ?1 AND `key` = ?2",
            (oci_layer_id, key),
            |row| row.get(0),
        )?;

        Ok(id)
    }

    /// List all annotations for a given layer.
    #[allow(dead_code)]
    pub(crate) fn list_by_layer(conn: &Connection, oci_layer_id: i64) -> anyhow::Result<Vec<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, oci_layer_id, `key`, `value`
             FROM oci_layer_annotation WHERE oci_layer_id = ?1 ORDER BY `key` ASC",
        )?;

        let rows = stmt.query_map([oci_layer_id], |row| {
            Ok(Self {
                id: row.get(0)?,
                oci_layer_id: row.get(1)?,
                key: row.get(2)?,
                value: row.get(3)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}
