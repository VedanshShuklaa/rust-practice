use std::fs;
use crate::models::Database;

const DB_FILE: &str = "db.json";

pub fn load_db() -> Database {
    match fs::read_to_string(DB_FILE) {
        Ok(data) => serde_json::from_str(&data).unwrap(),
        Err(_) => Database { current: None, users: vec![] },
    }
}

pub fn save_db(db: &Database) {
    let json = serde_json::to_string_pretty(db).unwrap();
    fs::write(DB_FILE, json).unwrap();
}
