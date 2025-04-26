use uuid::Uuid;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Business {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub owner_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}