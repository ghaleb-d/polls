use crate::models::{Choice, Poll};
use chrono::{Duration, Utc};
use std::io;
use uuid::Uuid;

pub fn create_poll() -> Result<Poll, io::Error> {
    println!("Enter your poll question:"); // Ask user to enter the poll question
    let mut question = String::new();
    io::stdin().read_line(&mut question)?;
    let question = question.trim().to_string(); // Clean whitespace

    // Ask how many choices (max 4)
    println!("How many choices? (Max 4):");
    let mut num_input = String::new();
    io::stdin().read_line(&mut num_input)?;

    // Convert user input to a number
    let num_choices: usize = num_input.trim().parse().unwrap_or(0);

    if num_choices == 0 || num_choices > 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid number of choices",
        ));
    }
    // Collect choices from user input
    let mut choices: Vec<Choice> = Vec::new();
    for i in 1..=num_choices {
        println!("Enter text for choice {}:", i);
        let mut choice_text = String::new();
        io::stdin().read_line(&mut choice_text)?;
        let choice_text = choice_text.trim().to_string();

        let choice = Choice {
            id: Uuid::new_v4(),
            text: choice_text,
            votes: 0,
        };

        choices.push(choice);
    }
    // Ask the user if he needs to add a deadline for his poll or not
    println!("Would you like to set a deadline for this poll");
    let mut is_deadline = String::new();
    io::stdin().read_line(&mut is_deadline)?;

    let is_deadline = is_deadline.trim().to_lowercase();
    // Declare a mutable variable to hold the deadline
    let mut deadline: Option<chrono::DateTime<Utc>> = None;

    match is_deadline.as_str() {
        "yes" => {
            println!("Please entre the number of days... Note the the maximum is 255 days");
            let mut days_str = String::new();
            io::stdin().read_line(&mut days_str)?;
            let number_days: u8 = days_str.trim().parse().unwrap_or(0); // This will fall back to zero to be reviewed
            // If days is more than 0, set the deadline
            if number_days > 0 {
                deadline = Some(Utc::now() + Duration::days(number_days as i64));
            } else {
                println!("Invalid number of days, deadline not set.");
            }
        }
        "no" => {
            deadline = None; // Explicitly setting no deadline
        }
        _ => {
            println!("Invalid answer. please answer by yes/no");
        }
    }

    // Construct and return the poll
    Ok(Poll {
        id: Uuid::new_v4(),
        question,
        choices,
        creation_time: Utc::now(),
        deadline,
    })
}
