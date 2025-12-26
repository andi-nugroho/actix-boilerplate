use actix_web::{web, HttpResponse, Result};
use crate::models::user::{CreateUserRequest, UpdateUserRequest};
use crate::services::user_service::UserService;

pub async fn get_users(user_service: web::Data<UserService>) -> Result<HttpResponse> {
    match user_service.get_all_users().await {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => {
            eprintln!("Error getting users: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to fetch users"))
        }
    }
}

pub async fn get_user(
    path: web::Path<String>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    
    match user_service.get_user_by_id(&user_id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Ok(HttpResponse::NotFound().json("User not found")),
        Err(e) => {
            eprintln!("Error getting user {}: {}", user_id, e);
            Ok(HttpResponse::InternalServerError().json("Failed to fetch user"))
        }
    }
}

pub async fn create_user(
    request: web::Json<CreateUserRequest>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse> {
    match user_service.create_user(request.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Created().json(user)),
        Err(e) => {
            eprintln!("Error creating user: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to create user"))
        }
    }
}

pub async fn update_user(
    path: web::Path<String>,
    request: web::Json<UpdateUserRequest>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    
    match user_service.update_user(&user_id, request.into_inner()).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Ok(HttpResponse::NotFound().json("User not found")),
        Err(e) => {
            eprintln!("Error updating user {}: {}", user_id, e);
            Ok(HttpResponse::InternalServerError().json("Failed to update user"))
        }
    }
}

pub async fn delete_user(
    path: web::Path<String>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    
    match user_service.delete_user(&user_id).await {
        Ok(Some(_)) => Ok(HttpResponse::Ok().json("User deleted successfully")),
        Ok(None) => Ok(HttpResponse::NotFound().json("User not found")),
        Err(e) => {
            eprintln!("Error deleting user {}: {}", user_id, e);
            Ok(HttpResponse::InternalServerError().json("Failed to delete user"))
        }
    }
}
