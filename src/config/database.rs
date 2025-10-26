use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn setup_database() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    ensure_database_exists(&database_url).await;

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(1)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

async fn ensure_database_exists(database_url: &str) {
    let (base_url, db_name) = database_url
        .rsplit_once('/')
        .expect("Invalid DATABASE_URL format");

    let admin_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(base_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    sqlx::query(&format!(
        "CREATE DATABASE \"{}\" WITH OWNER postgres",
        db_name
    ))
    .execute(&admin_pool)
    .await
    .ok(); // Ignore errors if DB already exists

    drop(admin_pool);
}
