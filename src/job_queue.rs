use sqlx::postgres::PgPool;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Job {
    pub id: Uuid,
    pub post_id: Uuid,
    pub platform_id: Uuid,
    pub status: String,
    pub scheduled_for: time::OffsetDateTime,
    pub attempt_count: Option<i32>,
    pub last_error: Option<String>,
    pub created_at: Option<time::OffsetDateTime>,
    pub processed_at: Option<time::OffsetDateTime>,
}

pub async fn poll_jobs(pool: PgPool) -> Result<Option<Job>, String> {
    // This query atomically selects and locks the next pending job that is ready to run
    // skipping any that are already locked by other workers.
    let job = sqlx::query_as::<_, Job>(
        r#"
        UPDATE jobs
        SET status = 'processing',
            attempt_count = COALESCE(attempt_count, 0) + 1,
            processed_at = NOW()
        WHERE id = (
            SELECT id
            FROM jobs
            WHERE status = 'pending'
            AND scheduled_for <= NOW()
            ORDER BY scheduled_for ASC
            FOR UPDATE SKIP LOCKED
            LIMIT 1
        )
        RETURNING *
        "#,
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(job)
}

#[derive(Debug, Clone, FromRow)]
pub struct JobPayload {
    pub job_id: Uuid,
    pub content: String,
    pub platform_name: String,
    pub credentials: sqlx::types::Json<serde_json::Value>,
    pub api_url: Option<String>,
    pub media_paths: Option<Vec<String>>,
}

pub async fn fetch_job_details(pool: PgPool, job_id: Uuid) -> Result<JobPayload, String> {
    sqlx::query_as::<_, JobPayload>(
        r#"
        SELECT 
            j.id as job_id,
            p.content as content,
            pl.name as platform_name,
            pl.credentials as credentials,
            pl.api_url as api_url,
            p.media_paths as media_paths
        FROM jobs j
        JOIN posts p ON j.post_id = p.id
        JOIN platforms pl ON j.platform_id = pl.id
        WHERE j.id = $1
        "#
    )
    .bind(job_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())
}
