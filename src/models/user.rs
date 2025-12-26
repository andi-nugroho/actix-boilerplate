use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Re-export Prisma user type
pub use crate::prisma::user;

// DTO for creating users
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

// DTO for updating users  
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

// DTO for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Convert Prisma user to response DTO
impl From<user::Data> for UserResponse {
    fn from(user: user::Data) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
