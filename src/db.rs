// Import necessary types and functions from the sqlx crate:
// - Pool: A connection pool abstraction
// - Postgres: Marker for using PostgreSQL
// - PgPoolOptions: Builder for configuring the pool
// Load environment variables from the `.env` file into std::env at runtime
// Used to access environment variables like DATABASE_URL
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

// Define a type alias for cleaner code throughout the app.
// Now we can write `DbPool` instead of `Pool<Postgres>`.
pub type DbPool = Pool<Postgres>;

// Asynchronously initialize a PostgreSQL connection pool.
// Returns a `Result` — Ok(DbPool) if successful, or a sqlx::Error on failure.
pub async fn init_pool() -> Result<DbPool, sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Build a connection pool with a max of 5 concurrent connections
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_init_pool_success() {
        let pool_result = init_pool().await;
        assert!(pool_result.is_ok(), "Failed to initialize pool");
    }
}
