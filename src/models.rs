use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// we need this attribute to be able to serialise/deserialize especially when we need to convert to/from JSON
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Poll {
    pub id: Uuid,             // Unique identifier for the poll (e.g., "f47ac10b-58cc...")
    pub question: String, // The main question of the poll (e.g., "What's your favorite programming language?")
    pub choices: Vec<Choice>, // A list of choices (the `Choice` struct is defined below)
    pub creation_time: DateTime<Utc>, // Timestamp for when the poll was created (in UTC)
    pub deadline: Option<DateTime<Utc>>, // Optional deadline for voting (if None, the poll is always open)
}

// Choice struct represents a single choice in a poll

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    pub id: Uuid,     // Unique identifier to prevent collisions
    pub text: String, // label of the choice or description
    pub votes: u32,   // unsigned and enough to track how many votes
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,         // could help avoiding duplicate voting and track votes per user
    pub username: String, // username as string
    pub user_creation_time: NaiveDateTime,
    pub voted_polls: Vec<Uuid>, // guaranteed to be non-null
}
