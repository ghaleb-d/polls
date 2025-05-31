// Import the shared database connection pool type
use crate::db::DbPool;
// Import the User struct definition
use crate::models::User;
// Utc for getting the current timestamp
use chrono::Utc;
// For reading from the terminal
use std::io;
// For generating unique user IDs
use uuid::Uuid;

// Asynchronous function to either fetch an existing user or create a new one
pub async fn create_user(pool: &DbPool) -> Result<User, sqlx::Error> {
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
    .fetch_optional(pool) // Returns Option<User>
    .await? // Propagates DB error if any
    {
        // If found, return the existing user
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
