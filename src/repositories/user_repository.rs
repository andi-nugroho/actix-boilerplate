use crate::models::user::{user, CreateUserRequest, UpdateUserRequest};
use crate::prisma::PrismaClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepository {
    client: Arc<PrismaClient>,
}

impl UserRepository {
    pub fn new(client: Arc<PrismaClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_users(&self) -> anyhow::Result<Vec<user::Data>> {
        let users = self
            .client
            .user()
            .find_many(vec![])
            .exec()
            .await?;
        
        Ok(users)
    }

    pub async fn get_user_by_id(&self, id: &str) -> anyhow::Result<Option<user::Data>> {
        let user = self
            .client
            .user()
            .find_unique(user::id::equals(id.to_string()))
            .exec()
            .await?;
        
        Ok(user)
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> anyhow::Result<user::Data> {
        let user = self
            .client
            .user()
            .create(request.name, request.email, vec![])
            .exec()
            .await?;
        
        Ok(user)
    }

    pub async fn update_user(&self, id: &str, request: UpdateUserRequest) -> anyhow::Result<Option<user::Data>> {
        let mut updates = vec![];
        
        if let Some(name) = request.name {
            updates.push(user::name::set(name));
        }
        
        if let Some(email) = request.email {
            updates.push(user::email::set(email));
        }

        if updates.is_empty() {
            return self.get_user_by_id(id).await;
        }

        let user = self
            .client
            .user()
            .update(user::id::equals(id.to_string()), updates)
            .exec()
            .await?;
        
        Ok(Some(user))
    }

    pub async fn delete_user(&self, id: &str) -> anyhow::Result<Option<user::Data>> {
        let user = self
            .client
            .user()
            .delete(user::id::equals(id.to_string()))
            .exec()
            .await?;
        
        Ok(Some(user))
    }
}
