use sqlx::{Pool, Postgres};

pub async fn init_db(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS workflows (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            nodes JSONB NOT NULL,
            edges JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS credentials (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            provider TEXT NOT NULL,
            data JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS mcp_servers (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            transport TEXT NOT NULL,
            command TEXT,
            args JSONB NOT NULL,
            endpoint TEXT,
            env JSONB NOT NULL,
            headers JSONB NOT NULL DEFAULT '{}',
            auto_start BOOLEAN NOT NULL,
            status TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Add headers column if it doesn't exist (for existing databases)
    let _ = sqlx::query(
        "ALTER TABLE mcp_servers ADD COLUMN IF NOT EXISTS headers JSONB NOT NULL DEFAULT '{}'"
    )
    .execute(pool)
    .await;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS executions (
            id UUID PRIMARY KEY,
            workflow_id TEXT NOT NULL,
            workflow_name TEXT NOT NULL,
            start_time TIMESTAMPTZ NOT NULL,
            end_time TIMESTAMPTZ,
            status TEXT NOT NULL,
            results JSONB NOT NULL,
            snapshot JSONB
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Migration for existing table
    let _ = sqlx::query("ALTER TABLE executions ADD COLUMN IF NOT EXISTS snapshot JSONB").execute(pool).await;

    Ok(())
}