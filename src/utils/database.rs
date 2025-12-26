use sqlx::PgPool;

pub fn init_database() {
    println!("🗄️ Database schema is managed by Prisma");
    println!("📋 Run 'prisma migrate dev' to apply migrations");
} 