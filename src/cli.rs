use crate::db::DbPool;
use crate::models::User;
use crate::polls::{create_poll, my_polls, view_polls, view_voted_pollts};
use crate::vote::vote_on_poll;
use colored::*;
use std::io;

pub async fn run_cli(pool: &DbPool, user: &mut User) -> Result<(), sqlx::Error> {
    loop {
        println!("{}", "\n🗳️ What would you like to do?".bold().underline());
        println!("{}", "1. Create a poll".yellow());
        println!("{}", "2. View polls".yellow());
        println!("{}", "3. View my created polls ".yellow());
        println!("{}", "4. View my voted polls ".yellow());
        println!("{}", "5. Vote on a poll".yellow());
        println!("{}", "6. Exit".yellow());

        let mut userchoice = String::new();
        io::stdin().read_line(&mut userchoice)?;

        match userchoice.trim() {
            "1" => {
                let poll = create_poll(pool, user).await?;
                println!("✅ Poll created: {:#?}", poll);
            }
            "2" => {
                println!("📋 View polls not implemented yet.");
                let polls = view_polls(pool).await?;
                if polls.is_empty() {
                    println!("📭 No polls found.");
                } else {
                    for (i, poll) in polls.iter().enumerate() {
                        println!("\nPoll #{}:", i + 1);
                        println!("📝 Question: {}", poll.question);
                        let total_votes: i32 = poll.vote_counts.iter().sum();
                        let max_votes = poll.vote_counts.iter().cloned().max().unwrap_or(0);
                        for (j, choice) in poll.choices.iter().enumerate() {
                            let count = poll.vote_counts[j];
                            let percentage = if total_votes > 0 {
                                (count as f64 / total_votes as f64) * 100.0
                            } else {
                                0.0
                            };
                            // Visual bar (1 block per 5%)
                            let bar_len = (percentage / 5.0).round() as usize;
                            let bar = "█".repeat(bar_len);

                            // Highlight the top choice
                            if count == max_votes && max_votes > 0 {
                                println!(
                                    "{}",
                                    format!(
                                        "  {}. {} — {} votes ({:.1}%) {} 🏆",
                                        j + 1,
                                        choice,
                                        count,
                                        percentage,
                                        bar
                                    )
                                    .green()
                                    .bold()
                                );
                            } else {
                                println!(
                                    "  {}. {} — {} votes ({:.1}%) {}",
                                    j + 1,
                                    choice,
                                    count,
                                    percentage,
                                    bar
                                );
                            }
                        }
                    }
                }
            }
            "3" => {
                println!("📋 We'll get your polls");
                let your_polls = my_polls(pool, user).await?;
                if your_polls.is_empty() {
                    println!("Sorry you have no polls");
                } else {
                    for (i, your_polls) in your_polls.iter().enumerate() {
                        let total_votes: i32 = your_polls.vote_counts.iter().sum();
                        println!("\nPoll #{}:", i + 1);
                        println!("📝 Question: {}", your_polls.question);
                        let max_votes = your_polls.vote_counts.iter().cloned().max().unwrap_or(0);
                        for (j, choice) in your_polls.choices.iter().enumerate() {
                            let count = your_polls.vote_counts[j];
                            let percentage = if total_votes > 0 {
                                (count as f64 / total_votes as f64) * 100.0
                            } else {
                                0.0
                            };
                            let bar_len = (percentage / 5.0).round() as usize;
                            let bar = "█".repeat(bar_len);
                            if count == max_votes && max_votes > 0 {
                                println!(
                                    "{}",
                                    format!(
                                        "  {}. {} — {} votes ({:.1}%) {} 🏆",
                                        j + 1,
                                        choice,
                                        count,
                                        percentage,
                                        bar
                                    )
                                    .green()
                                    .bold()
                                );
                            } else {
                                println!(
                                    "  {}. {} — {} votes ({:.1}%) {}",
                                    j + 1,
                                    choice,
                                    count,
                                    percentage,
                                    bar
                                );
                            }
                        }
                    }
                }
            }
            "4" => {
                println!("📋 We'll get you the polls you voted for");
                let voted_polls = view_voted_pollts(pool, user).await?;
                if voted_polls.is_empty() {
                    println!("Sorry you did not vote for any polls yet");
                } else {
                    for (i, voted_polls) in voted_polls.iter().enumerate() {
                        println!("\n #{},{:#?} :", i + 1, voted_polls);
                    }
                }
            }
            "5" => {
                vote_on_poll(pool, user).await?;
            }
            "6" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option. Try again."),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn calculates_percentage_correctly() {
        let count = 25;
        let total = 100;
        let percent = if total > 0 {
            (count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        assert_eq!(percent, 25.0);
    }
}
