pub mod models;

use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;

use crate::error::AppError;
use models::Workbook;

/// Database connection wrapper
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Connect to the database
    pub async fn connect(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Run database migrations
    pub async fn migrate(&self) -> anyhow::Result<()> {
        sqlx::migrate!("../../migrations").run(&self.pool).await?;
        Ok(())
    }

    /// List all workbooks
    pub async fn list_workbooks(&self) -> Result<Vec<Workbook>, AppError> {
        let workbooks = sqlx::query_as::<_, Workbook>(
            r#"SELECT id, name, created_at, updated_at FROM workbooks ORDER BY updated_at DESC"#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(workbooks)
    }

    /// Create a new workbook
    pub async fn create_workbook(&self, name: &str) -> Result<Workbook, AppError> {
        let workbook = sqlx::query_as::<_, Workbook>(
            r#"INSERT INTO workbooks (name) VALUES ($1) RETURNING id, name, created_at, updated_at"#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        Ok(workbook)
    }

    /// Get a workbook by ID
    pub async fn get_workbook(&self, id: Uuid) -> Result<Option<Workbook>, AppError> {
        let workbook = sqlx::query_as::<_, Workbook>(
            r#"SELECT id, name, created_at, updated_at FROM workbooks WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(workbook)
    }

    /// Update a workbook
    pub async fn update_workbook(&self, id: Uuid, name: &str) -> Result<Workbook, AppError> {
        let workbook = sqlx::query_as::<_, Workbook>(
            r#"UPDATE workbooks SET name = $1, updated_at = NOW() WHERE id = $2 RETURNING id, name, created_at, updated_at"#,
        )
        .bind(name)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(workbook)
    }

    /// Delete a workbook
    pub async fn delete_workbook(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM workbooks WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get workbook content as JSON
    pub async fn get_workbook_content(
        &self,
        id: Uuid,
    ) -> Result<Option<serde_json::Value>, AppError> {
        let result: Option<(Option<serde_json::Value>,)> = sqlx::query_as(
            r#"SELECT content FROM workbook_contents WHERE workbook_id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.and_then(|r| r.0))
    }

    /// Save workbook content as JSON
    pub async fn save_workbook_content(
        &self,
        id: Uuid,
        content: &serde_json::Value,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO workbook_contents (workbook_id, content)
            VALUES ($1, $2)
            ON CONFLICT (workbook_id)
            DO UPDATE SET content = $2, updated_at = NOW()
            "#,
        )
        .bind(id)
        .bind(content)
        .execute(&self.pool)
        .await?;

        // Also update the workbook's updated_at
        sqlx::query("UPDATE workbooks SET updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Store a CRDT update
    pub async fn store_yrs_update(&self, workbook_id: Uuid, update: &[u8]) -> Result<(), AppError> {
        sqlx::query("INSERT INTO yrs_updates (workbook_id, update_data) VALUES ($1, $2)")
            .bind(workbook_id)
            .bind(update)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get all CRDT updates for a workbook
    pub async fn get_yrs_updates(&self, workbook_id: Uuid) -> Result<Vec<Vec<u8>>, AppError> {
        let rows: Vec<(Vec<u8>,)> = sqlx::query_as(
            "SELECT update_data FROM yrs_updates WHERE workbook_id = $1 ORDER BY id",
        )
        .bind(workbook_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.0).collect())
    }
}
