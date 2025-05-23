use uuid::Uuid;
use serde_json::Value;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Activity {
    pub id: Uuid,
    pub business_id: Uuid,
    pub title: String,
    pub location: String,
    pub capacity: i32,
    pub time: chrono::DateTime<chrono::Utc>,
    pub equipment: Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}