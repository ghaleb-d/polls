use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// we need this attribute to be able to serialise/deserialize especially when we need to convert to/from JSON
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Poll {
    pub id: Uuid,
    pub question: String,
    pub choices: Vec<String>,
    pub vote_counts: Vec<i32>,
    pub creation_time: NaiveDateTime,
    pub deadline: Option<NaiveDateTime>,
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,         // could help avoiding duplicate voting and track votes per user
    pub username: String, // username as string
    pub user_creation_time: NaiveDateTime,
    pub voted_polls: Vec<Uuid>, // guaranteed to be non-null
}
