use uuid::Uuid;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Log {
    pub id: Uuid,
    pub level: String,
    pub message: String,
    pub context: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}