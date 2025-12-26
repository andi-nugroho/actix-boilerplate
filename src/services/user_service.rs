use crate::models::user::{user, CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::repositories::user_repository::UserRepository;

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn get_all_users(&self) -> anyhow::Result<Vec<UserResponse>> {
        let users = self.repository.get_all_users().await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    pub async fn get_user_by_id(&self, id: &str) -> anyhow::Result<Option<UserResponse>> {
        let user = self.repository.get_user_by_id(id).await?;
        Ok(user.map(UserResponse::from))
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<UserResponse> {
        let user = self.repository.create_user(request).await?;
        Ok(UserResponse::from(user))
    }

    pub async fn update_user(&self, id: &str, request: UpdateUserRequest) -> anyhow::Result<Option<UserResponse>> {
        let user = self.repository.update_user(id, request).await?;
        Ok(user.map(UserResponse::from))
    }

    pub async fn delete_user(&self, id: &str) -> anyhow::Result<Option<UserResponse>> {
        let user = self.repository.delete_user(id).await?;
        Ok(user.map(UserResponse::from))
    }
}
