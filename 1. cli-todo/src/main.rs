mod models;
mod storage;

use clap::{Parser, Subcommand};
use models::*;
use storage::*;

#[derive(Parser)]
#[command(name = "todo", version = "0.1")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Register {
        user: String,
        password: String,
        repeat: String,
    },
    Login {
        user: String,
        password: String,
    },
    Add {
        task: String,
    },
    List {},
    Complete {
        task_id: u16,
    },
    Logout,
}

fn main() {
    let args = Args::parse();
    let mut db = load_db();

    match args.command {
        Commands::Register { user, password, repeat } => {
            if password != repeat {
                println!("Passwords do not match");
                return;
            }

            if db.users.iter().any(|u| u.user == user) {
                println!("User already exists");
                return;
            }

            db.users.push(User {
                user,
                password,
                tasks: vec![],
            });

            save_db(&db);
            println!("User registered");
        }

        Commands::Login { user, password } => {
            let found = db.users.iter().find(|u| u.user == user && u.password == password);
            match found {
                Some(u) => {
                    db.current = Some(u.clone());
                    save_db(&db);
                    println!("Login successful");
                }
                None => println!("Invalid Credentials"),
            }
        }

        Commands::Add { task } => {
            if let Some(cur) = db.current.clone() {
                let username = cur.user;
                if let Some(user_mut) = find_user_mut(&mut db, &username) {
                    let id = user_mut.tasks.len() as u16 + 1;
                    user_mut.tasks.push(Task {
                        id,
                        task,
                        priority: Priority::Medium,
                        status: Status::Pending,
                        tags: vec![],
                    });
                    db.current = Some(user_mut.clone());
                    save_db(&db);
                    println!("Task added");
                } else {
                    println!("Current user not found in DB");
                }
            } else {
                println!("Login First!");
            }
        }

        Commands::List {} => {
            if let Some(cur) = &db.current {
                if let Some(user_ref) = find_user(&db, &cur.user) {
                    for t in &user_ref.tasks {
                        println!("[{}] {:?} - {}", t.id, t.status, t.task);
                    }
                } else {
                    println!("Current user not found");
                }
            } else {
                println!("Login First!");
            }
        }

        Commands::Complete { task_id } => {
            if let Some(cur) = db.current.clone() {
                let username = cur.user;
                if let Some(user_mut) = find_user_mut(&mut db, &username) {
                    if let Some(task) = user_mut.tasks.iter_mut().find(|t| t.id == task_id) {
                        task.status = Status::Completed;
                        db.current = Some(user_mut.clone());
                        save_db(&db);
                        println!("Task completed");
                    } else {
                        println!("Task not found");
                    }
                } else {
                    println!("Current user not found in DB");
                }
            } else {
                println!("Login First!");
            }
        }

        Commands::Logout => {
            db.current = None;
            save_db(&db);
            println!("Logged out");
        }
    }
}

fn find_user<'a>(db: &'a Database, user: &str) -> Option<&'a User> {
    db.users.iter().find(|u| u.user == user)
}

fn find_user_mut<'a>(db: &'a mut Database, user: &str) -> Option<&'a mut User> {
    db.users.iter_mut().find(|u| u.user == user)
}