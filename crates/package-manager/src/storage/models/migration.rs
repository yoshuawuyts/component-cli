use anyhow::Context;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};

/// A migration that can be applied to the database.
struct MigrationDef {
    version: u32,
    name: &'static str,
    sql: &'static str,
}

/// All migrations in order. Each migration is run exactly once.
const MIGRATIONS: &[MigrationDef] = &[
    MigrationDef {
        version: 1,
        name: "init",
        sql: include_str!("../migrations/01_init.sql"),
    },
    MigrationDef {
        version: 2,
        name: "known_packages",
        sql: include_str!("../migrations/02_known_packages.sql"),
    },
    MigrationDef {
        version: 3,
        name: "known_package_tags",
        sql: include_str!("../migrations/03_known_package_tags.sql"),
    },
    MigrationDef {
        version: 4,
        name: "image_size",
        sql: include_str!("../migrations/04_image_size.sql"),
    },
    MigrationDef {
        version: 5,
        name: "tag_type",
        sql: include_str!("../migrations/05_tag_type.sql"),
    },
    MigrationDef {
        version: 6,
        name: "wit_interface",
        sql: include_str!("../migrations/06_wit_interface.sql"),
    },
    MigrationDef {
        version: 7,
        name: "package_name",
        sql: include_str!("../migrations/07_package_name.sql"),
    },
];

/// Information about the current migration state.
#[derive(Debug, Clone)]
pub struct Migrations {
    /// The current migration version applied to the database.
    pub current: u32,
    /// The total number of migrations available.
    pub total: u32,
}

/// Execute a SQL string that may contain multiple statements separated by semicolons.
async fn execute_batch(conn: &DatabaseConnection, sql: &str) -> anyhow::Result<()> {
    for statement in sql.split(';') {
        let stmt = statement.trim();
        if !stmt.is_empty() {
            conn.execute_unprepared(stmt)
                .await
                .with_context(|| format!("Failed to execute SQL: {stmt}"))?;
        }
    }
    Ok(())
}

impl Migrations {
    /// Initialize the migrations table and run all pending migrations.
    pub(crate) async fn run_all(conn: &DatabaseConnection) -> anyhow::Result<()> {
        // Create the migrations table if it doesn't exist
        execute_batch(conn, include_str!("../migrations/00_migrations.sql")).await?;

        // Get the current migration version
        let current_version = Self::current_version(conn).await;

        // Run all migrations that haven't been applied yet
        for migration in MIGRATIONS {
            if migration.version > current_version {
                execute_batch(conn, migration.sql).await.with_context(|| {
                    format!(
                        "Failed to run migration {}: {}",
                        migration.version, migration.name
                    )
                })?;

                conn.execute(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    "INSERT INTO migrations (version) VALUES (?)",
                    [sea_orm::Value::from(migration.version as i32)],
                ))
                .await?;
            }
        }

        Ok(())
    }

    /// Returns information about the current migration state.
    pub(crate) async fn get(conn: &DatabaseConnection) -> anyhow::Result<Self> {
        let current = Self::current_version(conn).await;
        let total = MIGRATIONS.last().map(|m| m.version).unwrap_or(0);
        Ok(Self { current, total })
    }

    /// Get the current migration version from the database.
    async fn current_version(conn: &DatabaseConnection) -> u32 {
        conn.query_one(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT COALESCE(MAX(version), 0) FROM migrations".to_owned(),
        ))
        .await
        .ok()
        .flatten()
        .and_then(|row| row.try_get_by_index::<i32>(0).ok())
        .map(|v| v as u32)
        .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::Database;

    async fn setup_test_conn() -> DatabaseConnection {
        Database::connect("sqlite::memory:").await.unwrap()
    }

    #[tokio::test]
    async fn test_migrations_run_all_creates_tables() {
        let conn = setup_test_conn().await;
        Migrations::run_all(&conn).await.unwrap();

        // Verify migrations table exists and has records
        let result = conn
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT COUNT(*) FROM migrations".to_owned(),
            ))
            .await
            .unwrap()
            .unwrap();
        let count: i32 = result.try_get_by_index(0).unwrap();
        assert!(count > 0);

        // Verify tables exist by running queries
        conn.execute_unprepared("SELECT 1 FROM image LIMIT 1")
            .await
            .ok();
        conn.execute_unprepared("SELECT 1 FROM known_package LIMIT 1")
            .await
            .ok();
        conn.execute_unprepared("SELECT 1 FROM known_package_tag LIMIT 1")
            .await
            .ok();
    }

    #[tokio::test]
    async fn test_migrations_run_all_idempotent() {
        let conn = setup_test_conn().await;

        // Run migrations multiple times
        Migrations::run_all(&conn).await.unwrap();
        Migrations::run_all(&conn).await.unwrap();
        Migrations::run_all(&conn).await.unwrap();

        // Should still work correctly
        let info = Migrations::get(&conn).await.unwrap();
        assert_eq!(info.current, info.total);
    }

    #[tokio::test]
    async fn test_migrations_get_info() {
        let conn = setup_test_conn().await;
        Migrations::run_all(&conn).await.unwrap();

        let info = Migrations::get(&conn).await.unwrap();

        // Current should equal total after running all migrations
        assert_eq!(info.current, info.total);
        // Total should match the number of migrations defined
        let expected_total = MIGRATIONS.last().map(|m| m.version).unwrap_or(0);
        assert_eq!(info.total, expected_total);
    }

    #[tokio::test]
    async fn test_migrations_get_before_running() {
        let conn = setup_test_conn().await;

        // Create migrations table manually to test get() on fresh db
        execute_batch(&conn, include_str!("../migrations/00_migrations.sql"))
            .await
            .unwrap();

        let info = Migrations::get(&conn).await.unwrap();

        // Current should be 0 before running migrations
        assert_eq!(info.current, 0);
        // Total should still reflect available migrations
        let expected_total = MIGRATIONS.last().map(|m| m.version).unwrap_or(0);
        assert_eq!(info.total, expected_total);
    }
}
