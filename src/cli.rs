use crate::db::DbPool;
use crate::models::User;
use crate::polls::{create_poll, my_polls, view_polls};
use std::io;
use crate::vote::vote_on_poll;

pub async fn run_cli(pool: &DbPool, user: &mut User) -> Result<(), sqlx::Error> {
    loop {
        println!("\n🗳️ What would you like to do?");
        println!("1. Create a poll");
        println!("2. View polls");
        println!("3. View my created polls ");
        println!("4. Vote on a poll");
        println!("5. Exit");

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
                        for (j, choice) in poll.choices.iter().enumerate() {
                            println!("  {}. {} — {} votes", j + 1, choice, poll.vote_counts[j]);
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
                        println!("\n #{},{:#?} :", i + 1, your_polls);
                    }
                }
            }
            "4" => {
                vote_on_poll(pool, user).await?;
            }
            "5" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option. Try again."),
        }
    }

    Ok(())
}
