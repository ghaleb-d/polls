use crate::db::DbPool;
use crate::models::User;
use crate::polls::view_polls;
use std::io;

// Function to handle user voting on a poll.
// It fetches polls from the database, checks for duplicates, updates vote count, and records the user's vote.
pub async fn vote_on_poll(pool: &DbPool, user: &mut User) -> Result<(), sqlx::Error> {
    // Step 1: Fetch all polls from the database
    let polls = view_polls(pool).await?;
    if polls.is_empty() {
        println!("📭 No polls available.");
        return Ok(()); // Nothing to vote on
    }

    // Step 2: Display all poll questions with numbers
    println!("Available Polls:");
    for (i, poll) in polls.iter().enumerate() {
        println!("{}. {}", i + 1, poll.question);
    }

    // Step 3: Ask the user to select a poll by number
    println!("Enter the number of the poll you want to vote on:");
    let mut poll_input = String::new();
    io::stdin().read_line(&mut poll_input)?;
    let selected_index: usize = poll_input.trim().parse().unwrap_or(0);

    // Validate poll selection
    if selected_index == 0 || selected_index > polls.len() {
        println!("❌ Invalid poll number.");
        return Ok(());
    }

    // Get the selected poll based on user input
    let selected_poll = &polls[selected_index - 1];

    // Step 4: Check if the user has already voted on this poll
    if user.voted_polls.contains(&selected_poll.id) {
        println!("❌ You have already voted in this poll.");
        return Ok(());
    }

    // Step 5: Show choices for the selected poll
    println!("📝 Poll: {}", selected_poll.question);
    for (i, choice) in selected_poll.choices.iter().enumerate() {
        println!("{}. {}", i + 1, choice);
    }

    // Step 6: Ask for user's vote (choice number)
    println!("Enter the number of your choice:");
    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input)?;
    let choice_index: usize = choice_input.trim().parse().unwrap_or(0);

    // Validate choice selection
    if choice_index == 0 || choice_index > selected_poll.choices.len() {
        println!("❌ Invalid choice number.");
        return Ok(());
    }

    // Step 7: Update vote count for the selected choice
    let mut updated_counts = selected_poll.vote_counts.clone();
    updated_counts[choice_index - 1] += 1;

    // Update the poll in the DB with the new vote counts
    sqlx::query!(
        r#"
        UPDATE polls SET vote_counts = $1 WHERE id = $2
        "#,
        &updated_counts,
        selected_poll.id
    )
    .execute(pool)
    .await?;

    // Step 8: Record that the user has voted in this poll
    let mut updated_voted = user.voted_polls.clone();
    updated_voted.push(selected_poll.id);

    // Update the user's voted_polls array in the DB
    sqlx::query!(
        r#"
        UPDATE users SET voted_polls = $1 WHERE id = $2
        "#,
        &updated_voted,
        user.id
    )
    .execute(pool)
    .await?;

    // Also update the in-memory user struct so it's accurate for this session
    user.voted_polls = updated_voted;

    // Step 9: Confirm to the user that their vote has been recorded
    println!(
        "✅ Your vote for \"{}\" has been recorded!",
        selected_poll.choices[choice_index - 1]
    );

    Ok(())
}
