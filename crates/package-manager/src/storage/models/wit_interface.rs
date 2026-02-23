use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, Value};

/// A WIT interface extracted from a WebAssembly component.
#[derive(Debug, Clone)]
pub struct WitInterface {
    id: i64,
    /// The package name (e.g., "wasi:http@0.2.0")
    pub package_name: Option<String>,
    /// The full WIT text representation
    pub wit_text: String,
    /// The world name if available
    pub world_name: Option<String>,
    /// Number of imports
    pub import_count: i32,
    /// Number of exports
    pub export_count: i32,
    /// When this was created
    pub created_at: String,
}

impl WitInterface {
    /// Returns the ID of this WIT interface.
    #[must_use]
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Create a new WitInterface for testing purposes
    #[must_use]
    pub fn new_for_testing(
        id: i64,
        package_name: Option<String>,
        wit_text: String,
        world_name: Option<String>,
        import_count: i32,
        export_count: i32,
        created_at: String,
    ) -> Self {
        Self {
            id,
            package_name,
            wit_text,
            world_name,
            import_count,
            export_count,
            created_at,
        }
    }

    /// Insert a new WIT interface and return its ID.
    /// Uses content-addressable storage - if the same WIT text already exists, returns existing ID.
    pub(crate) async fn insert(
        conn: &DatabaseConnection,
        wit_text: &str,
        package_name: Option<&str>,
        world_name: Option<&str>,
        import_count: i32,
        export_count: i32,
    ) -> anyhow::Result<i64> {
        // Check if this exact WIT text already exists
        let existing = conn
            .query_one(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT id FROM wit_interface WHERE wit_text = ?",
                vec![Value::from(wit_text.to_string())],
            ))
            .await?;

        if let Some(row) = existing {
            return Ok(row.try_get_by_index::<i64>(0)?);
        }

        // Insert new WIT interface
        conn.execute(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "INSERT INTO wit_interface (wit_text, package_name, world_name, import_count, export_count) VALUES (?, ?, ?, ?, ?)",
            vec![
                Value::from(wit_text.to_string()),
                Value::from(package_name.map(|s| s.to_string())),
                Value::from(world_name.map(|s| s.to_string())),
                Value::from(import_count),
                Value::from(export_count),
            ],
        ))
        .await?;

        // Get the last inserted ID
        let row = conn
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT last_insert_rowid()".to_owned(),
            ))
            .await?
            .expect("last_insert_rowid should always return a row");

        Ok(row.try_get_by_index::<i64>(0)?)
    }

    /// Link an image to a WIT interface.
    pub(crate) async fn link_to_image(
        conn: &DatabaseConnection,
        image_id: i64,
        wit_interface_id: i64,
    ) -> anyhow::Result<()> {
        conn.execute(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "INSERT OR IGNORE INTO image_wit_interface (image_id, wit_interface_id) VALUES (?, ?)",
            vec![Value::from(image_id), Value::from(wit_interface_id)],
        ))
        .await?;
        Ok(())
    }

    /// Get WIT interface for an image by image ID.
    #[allow(dead_code)]
    pub(crate) async fn get_for_image(
        conn: &DatabaseConnection,
        image_id: i64,
    ) -> anyhow::Result<Option<Self>> {
        let row = conn
            .query_one(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT w.id, w.package_name, w.wit_text, w.world_name, w.import_count, w.export_count, w.created_at
                 FROM wit_interface w
                 JOIN image_wit_interface iwi ON w.id = iwi.wit_interface_id
                 WHERE iwi.image_id = ?",
                vec![Value::from(image_id)],
            ))
            .await?;

        match row {
            Some(row) => Ok(Some(Self::from_query_result(&row)?)),
            None => Ok(None),
        }
    }

    /// Get all WIT interfaces with their associated image references.
    pub(crate) async fn get_all_with_images(
        conn: &DatabaseConnection,
    ) -> anyhow::Result<Vec<(Self, String)>> {
        let rows = conn
            .query_all(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT w.id, w.package_name, w.wit_text, w.world_name, w.import_count, w.export_count, w.created_at,
                        i.ref_registry || '/' || i.ref_repository || COALESCE(':' || i.ref_tag, '') as reference
                 FROM wit_interface w
                 JOIN image_wit_interface iwi ON w.id = iwi.wit_interface_id
                 JOIN image i ON iwi.image_id = i.id
                 ORDER BY w.package_name ASC, w.world_name ASC, i.ref_repository ASC",
                vec![],
            ))
            .await?;

        let mut result = Vec::new();
        for row in &rows {
            let interface = Self::from_query_result(row)?;
            let reference: String = row.try_get_by_index(7)?;
            result.push((interface, reference));
        }
        Ok(result)
    }

    /// Get all unique WIT interfaces.
    #[allow(dead_code)]
    pub(crate) async fn get_all(conn: &DatabaseConnection) -> anyhow::Result<Vec<Self>> {
        let rows = conn
            .query_all(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT id, package_name, wit_text, world_name, import_count, export_count, created_at
                 FROM wit_interface
                 ORDER BY package_name ASC, world_name ASC",
                vec![],
            ))
            .await?;

        let mut result = Vec::new();
        for row in &rows {
            result.push(Self::from_query_result(row)?);
        }
        Ok(result)
    }

    /// Delete a WIT interface by ID (also removes links).
    #[allow(dead_code)]
    pub(crate) async fn delete(conn: &DatabaseConnection, id: i64) -> anyhow::Result<bool> {
        let result = conn
            .execute(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "DELETE FROM wit_interface WHERE id = ?",
                vec![Value::from(id)],
            ))
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Helper to construct a WitInterface from a query result row.
    fn from_query_result(row: &sea_orm::QueryResult) -> anyhow::Result<Self> {
        Ok(Self {
            id: row.try_get_by_index(0)?,
            package_name: row.try_get_by_index(1)?,
            wit_text: row.try_get_by_index(2)?,
            world_name: row.try_get_by_index(3)?,
            import_count: row.try_get_by_index(4)?,
            export_count: row.try_get_by_index(5)?,
            created_at: row.try_get_by_index(6)?,
        })
    }
}
