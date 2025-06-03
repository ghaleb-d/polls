use crate::db::DbPool;
use crate::models::{Poll, User};
use chrono::{Duration, Utc};
use sqlx::Error;
use std::io;
use uuid::Uuid;

pub async fn create_poll(pool: &DbPool, user: &User) -> Result<Poll, Error> {
    println!("Enter your poll question:");
    let mut question = String::new();
    io::stdin().read_line(&mut question)?;
    let question = question.trim().to_string();

    println!("How many choices? (Max 4):");
    let mut num_input = String::new();
    io::stdin().read_line(&mut num_input)?;
    let num_choices: usize = num_input.trim().parse().unwrap_or(0);
    if num_choices == 0 || num_choices > 4 {
        return Err(Error::ColumnNotFound("Invalid number of choices".into()));
    }

    let mut choices = Vec::new();
    for i in 1..=num_choices {
        println!("Enter text for choice {}:", i);
        let mut choice_text = String::new();
        io::stdin().read_line(&mut choice_text)?;
        choices.push(choice_text.trim().to_string());
    }

    println!("Would you like to set a deadline for this poll? (yes/no):");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    let answer = answer.trim().to_lowercase();
    let mut deadline = None;

    if answer == "yes" {
        println!("Enter number of days (max 255):");
        let mut days = String::new();
        io::stdin().read_line(&mut days)?;
        let number_days: u8 = days.trim().parse().unwrap_or(0);
        if number_days > 0 {
            deadline = Some((Utc::now() + Duration::days(number_days as i64)).naive_utc());
        }
    }

    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();
    let vote_counts: Vec<i32> = vec![0; choices.len()];

    let poll = sqlx::query_as!(
        Poll,
        r#"
        INSERT INTO polls (id, question, choices, vote_counts, creation_time, deadline, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, question, choices, vote_counts, creation_time, deadline, created_by
        "#,
        id,
        question,
        &choices,
        &vote_counts,
        now,
        deadline,
        user.id
    )
    .fetch_one(pool)
    .await?;

    println!("✅ Poll created successfully.");

    Ok(poll)
}

pub async fn view_polls(pool: &DbPool) -> Result<Vec<Poll>, Error> {
    let polls = sqlx::query_as!(
        Poll,
        r#"
        SELECT id, question, choices, vote_counts, creation_time, deadline, created_by
        FROM polls
        ORDER BY creation_time DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(polls)
}

pub async fn my_polls(pool: &DbPool, user: &User) -> Result<Vec<Poll>, Error> {
    let my_polls = sqlx::query_as!(
        Poll,
        r#"
        SELECT id, question, choices, vote_counts, creation_time, deadline, created_by
        FROM polls
        WHERE created_by = $1
        ORDER BY creation_time DESC
        "#,
        user.id
    )
    .fetch_all(pool)
    .await?;

    Ok(my_polls)
}

pub async fn view_voted_pollts(pool: &DbPool, user: &User) -> Result<Vec<Poll>, Error> {
    if user.voted_polls.is_empty() {
        return Ok(vec![]);
    }
    let polls = sqlx::query_as!(
        Poll,
        r#"
        SELECT id, question, choices, vote_counts, creation_time, deadline, created_by
        FROM polls
        WHERE id = ANY($1)
        ORDER BY creation_time DESC
        "#,
        &user.voted_polls
    )
    .fetch_all(pool)
    .await?;

    Ok(polls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::User;
    use chrono::Utc;
    use dotenv::dotenv;
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use std::env;
    use uuid::Uuid;

    async fn setup_test_db() -> PgPool {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPoolOptions::new()
            .max_connections(1)
            .connect(&db_url)
            .await
            .expect("Failed to connect to test DB")
    }

    async fn create_test_user(pool: &PgPool) -> User {
        let username = format!("testuser_{}", Uuid::new_v4());
        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();

        sqlx::query_as!(
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
        .await
        .expect("Failed to insert test user")
    }

    async fn create_poll_with_data(
        pool: &DbPool,
        user: &User,
        question: &str,
        choices: Vec<String>,
        deadline_days: Option<u8>,
    ) -> Poll {
        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();
        let vote_counts = vec![0; choices.len()];
        let deadline = deadline_days.map(|d| (Utc::now() + Duration::days(d as i64)).naive_utc());

        sqlx::query_as!(
            Poll,
            r#"
            INSERT INTO polls (id, question, choices, vote_counts, creation_time, deadline, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, question, choices, vote_counts, creation_time, deadline, created_by
            "#,
            id,
            question,
            &choices,
            &vote_counts,
            now,
            deadline,
            user.id
        )
        .fetch_one(pool)
        .await
        .expect("Failed to insert poll")
    }

    #[tokio::test]
    async fn test_create_poll_and_view() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        let poll = create_poll_with_data(
            &pool,
            &user,
            "What's your favorite Rust feature?",
            vec!["Ownership".into(), "Borrow Checker".into()],
            Some(7),
        )
        .await;

        assert_eq!(poll.question, "What's your favorite Rust feature?");
        assert_eq!(poll.choices.len(), 2);
        assert_eq!(poll.vote_counts, vec![0, 0]);
        assert_eq!(poll.created_by, user.id);
    }

    #[tokio::test]
    async fn test_my_polls_returns_user_polls() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        create_poll_with_data(
            &pool,
            &user,
            "Test Poll",
            vec!["A".into(), "B".into()],
            None,
        )
        .await;

        let polls = my_polls(&pool, &user).await.unwrap();

        assert!(!polls.is_empty());
        assert_eq!(polls[0].created_by, user.id);
    }

    #[tokio::test]
    async fn test_view_polls_returns_all_polls() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        create_poll_with_data(
            &pool,
            &user,
            "Public Poll",
            vec!["A".into(), "B".into()],
            None,
        )
        .await;

        let polls = view_polls(&pool).await.unwrap();

        assert!(!polls.is_empty());
    }

    #[tokio::test]
    async fn test_view_voted_polls_returns_voted_only() {
        let pool = setup_test_db().await;
        let mut user = create_test_user(&pool).await;

        let poll = create_poll_with_data(
            &pool,
            &user,
            "Vote Tracking",
            vec!["X".into(), "Y".into()],
            None,
        )
        .await;

        // simulate vote
        user.voted_polls.push(poll.id);

        let voted = view_voted_pollts(&pool, &user).await.unwrap();
        assert_eq!(voted.len(), 1);
        assert_eq!(voted[0].id, poll.id);
    }
}


