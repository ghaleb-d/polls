// Import the shared database connection pool type
use crate::db::DbPool;
// Import the User struct definition
use crate::models::User;
// Utc for getting the current timestamp
use chrono::Utc;
// For reading from the terminal
use std::io;
// For generating unique user IDs
use sqlx::Error;
use uuid::Uuid;

// This function decides whether the user wants to log in or create a new account.
// It returns a Result<User, sqlx::Error> after calling either `load_user` or `create_user`.
pub async fn choose_user_flow(pool: &DbPool) -> Result<User, Error> {
    // Prompt the user with a yes/no question
    println!("Do you have an existing username? (yes/no):");

    // Prepare a mutable String to read input into
    let mut answer = String::new();

    // Read the user's input from the terminal (e.g., "yes" or "no")
    io::stdin().read_line(&mut answer)?;

    // Clean up input by trimming whitespace and converting to lowercase
    let answer = answer.trim().to_lowercase();

    // Use match to handle the input value
    match answer.as_str() {
        "yes" => {
            // If user says "yes", try to load the user from the DB
            load_user(pool).await
        }
        "no" => {
            // If user says "no", prompt for a new username and create the user in the DB
            create_user(pool).await
        }
        _ => {
            // If user types anything else, show an error message
            println!("❌ Invalid input. Please answer with 'yes' or 'no'.");

            // Return an error so that main.rs can handle it
            Err(Error::ColumnNotFound("Invalid yes/no response".into()))
        }
    }
}

// This function is used to authenticate a user by checking if the entered username already exists in the database.
async fn load_user(pool: &DbPool) -> Result<User, sqlx::Error> {
    // Prompt the user to enter their username
    println!("Enter your username to log in:");

    // Prepare a String to hold the input
    let mut username = String::new();

    // Read input from the terminal and store it in the `username` variable
    io::stdin().read_line(&mut username)?;

    // Clean up the input:
    // - remove newline/whitespace with trim()
    // - normalize to lowercase for case-insensitive matching
    let username = username.trim().to_lowercase();

    // Validate: if the username is empty, return an error immediately
    if username.is_empty() {
        return Err(sqlx::Error::ColumnNotFound("Username is empty".into()));
    }

    // Attempt to fetch the user from the database using their username
    // - query_as! maps the result row to your `User` struct
    // - $1 is a placeholder for the first argument passed (here: `username`)
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, user_creation_time, voted_polls
        FROM users
        WHERE username = $1
        "#,
        username
    )
    // `fetch_optional()` returns an Option<User> — Some(user) if found, None if not
    .fetch_optional(pool)
    .await?;

    // Handle the result:
    // - If the user exists, return it (success)
    // - If not, print a message and return an error
    match user {
        Some(user) => Ok(user),
        None => {
            println!("❌ No user found with that username.");
            Err(Error::RowNotFound)
        }
    }
}

// Asynchronous function to either fetch an existing user or create a new one
async fn create_user(pool: &DbPool) -> Result<User, sqlx::Error> {
    // Prompt the user to enter a username
    println!("Please enter a username");

    // Create a mutable String to hold input
    let mut username = String::new();

    // Read user input from stdin
    io::stdin().read_line(&mut username)?;

    // Clean up the input: remove whitespace and convert to lowercase
    let username = username.trim().to_lowercase();

    // If the user just hits enter (empty string), return an error
    if username.is_empty() {
        return Err(sqlx::Error::ColumnNotFound("Username is empty".into()));
    }

    // Try to find an existing user with this username in the database
    if let Some(user) = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, user_creation_time, voted_polls
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?
    {
        // If found, return the existing user
        println!("this user already exists{}", username);
        return Ok(user);
    }

    // If the user doesn't exist, generate a new ID and current timestamp
    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc(); // Convert to timezone-less timestamp for Postgres

    // Insert the new user into the database and return the inserted row
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, username, user_creation_time, voted_polls)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, user_creation_time, voted_polls
        "#,
        id,
        username,
        now,
        &[] // Start with an empty array of voted polls
    )
    .fetch_one(pool) // This returns exactly one row
    .await?; // Propagate DB error if the query fails

    // Return the newly created user
    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use dotenv::dotenv;
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use std::env;
    use tokio; // ✅ Needed for #[tokio::test]
    use uuid::Uuid;

    async fn setup_test_db() -> PgPool {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPoolOptions::new()
            .max_connections(1)
            .connect(&db_url)
            .await
            .expect("Could not connect to test DB")
    }

    #[tokio::test]
    async fn test_load_user_success() {
        let pool = setup_test_db().await;

        let username = format!("testuser_{}", Uuid::new_v4());
        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();

        // insert test user
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, user_creation_time, voted_polls)
            VALUES ($1, $2, $3, $4)
            "#,
            id,
            username,
            now,
            &[]
        )
        .execute(&pool)
        .await
        .expect("Insert failed");

        // simulate load_user logic
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, user_creation_time, voted_polls
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_one(&pool)
        .await
        .expect("Should load the user");

        assert_eq!(user.username, username);
    }

    // New testable function
    pub async fn create_user_with_name(pool: &DbPool, username: &str) -> Result<User, sqlx::Error> {
        let username = username.trim().to_lowercase();

        if username.is_empty() {
            return Err(sqlx::Error::ColumnNotFound("Username is empty".into()));
        }

        if let Some(user) = sqlx::query_as!(
            User,
            r#"
        SELECT id, username, user_creation_time, voted_polls
        FROM users
        WHERE username = $1
        "#,
            username
        )
        .fetch_optional(pool)
        .await?
        {
            return Ok(user);
        }

        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();

        let user = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (id, username, user_creation_time, voted_polls)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, user_creation_time, voted_polls
        "#,
            id,
            username,
            now,
            &[]
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    #[tokio::test]
    async fn test_create_user_empty_input_returns_error() {
        let pool = setup_test_db().await;

        let result = create_user_with_name(&pool, "").await;

        match result {
            Err(sqlx::Error::ColumnNotFound(msg)) => {
                assert_eq!(msg, "Username is empty");
            }
            _ => panic!("Expected ColumnNotFound error, got {:?}", result),
        }
    }
}
