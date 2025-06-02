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
