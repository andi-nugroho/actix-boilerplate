use actix_web::{web, App, HttpServer, middleware::Logger};
use crud_test::*;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize environment and logging
    env_logger::init();
    
    // Load configuration
    let config = config::AppConfig::from_env()?;
    println!("🚀 Starting server at {}:{}", config.server_host, config.server_port);
    
    // Create database connection pool
    let pool = config.create_db_pool().await?;
    println!("🗄️ Connected to database successfully!");
    
    // Run database migrations
    utils::run_migrations(&pool).await?;
    
    // Initialize repositories
    let user_repository = repositories::UserRepository::new(pool.clone());
    let user_service = services::UserService::new(user_repository);
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(note_service.clone()))
            .app_data(web::Data::new(money_service.clone()))
            .service(
                web::scope("/api/users")
                    .route("", web::get().to(controllers::get_users))
                    .route("", web::post().to(controllers::create_user))
                    .route("/{id}", web::get().to(controllers::get_user))
                    .route("/{id}", web::put().to(controllers::update_user))
                    .route("/{id}", web::delete().to(controllers::delete_user))
            )
    })
    .bind(format!("{}:{}", config.server_host, config.server_port))?
    .run()
    .await?;
    
    Ok(())
}