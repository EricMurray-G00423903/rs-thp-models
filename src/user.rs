use uuid::Uuid;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        PublicUser { 
            id: user.id, 
            email: user.email, 
            created_at: user.created_at 
        }
    }
}