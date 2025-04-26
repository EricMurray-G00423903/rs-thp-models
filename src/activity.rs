use uuid::Uuid;
use serde_json::Value;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Activity {
    pub id: Uuid,
    pub business_id: Uuid,
    pub title: String,
    pub location: String,
    pub capacity: u32,
    pub time: chrono::DateTime<chrono::Utc>,
    pub equipment: Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}