use uuid::Uuid;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
 pub struct Booking  {
    pub id: Uuid,
    pub user_id: Uuid,
    pub activity_id: Uuid,
    pub booked_at: chrono::DateTime<chrono::Utc>,
}