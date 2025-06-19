use sqlx::PgPool;

pub async fn initialize_pool() -> PgPool {
    let pool = PgPool::connect(
        std::env::var("DATABASE_URL")
            .unwrap_or(String::from("postgres://devuser:devpass@localhost:5432/devdb"))
            .as_str()
    )
        .await
        .expect("failed to connect to database");
    println!("database initialized");
    pool
}