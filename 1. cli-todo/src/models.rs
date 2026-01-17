use serde::{Serialize, Deserialize};
use clap::ValueEnum;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Pending,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u16,
    pub task: String,
    pub priority: Priority,
    pub status: Status,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user: String,
    pub password: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub current: Option<User>,
    pub users: Vec<User>,
}