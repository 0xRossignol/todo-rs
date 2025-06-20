use std::process;
use crate::task;
use crate::task::{Database, Status};

pub fn add(args: &Vec<String>, mut db: Database) {
    let contents = &args[2..].join(" ");
    let id = db.read_tasks().unwrap().last().map(|task| task.id + 1).unwrap_or(0);
    let status = Status::TODO;
    if let Err(e) = db.add_task(task::Task {
        id,
        content: contents.to_string(),
        status,
    }) {
        eprintln!("Error adding task: {}", e);
        process::exit(1);
    }
}

pub fn ls(mut db: Database) {
    let tasks = db.read_tasks().unwrap_or_else(|err| {
        eprintln!("Error reading tasks: {}", err);
        process::exit(1);
    });
    if tasks.is_empty() {
        eprintln!("No tasks found, you can add one using `rodo add <task>`");
        return;
    }

    for task in tasks {
        match task.status {
            Status::TODO => println!("[]\t{}\t{}", task.id, task.content),
            Status::IN_PROGRESS => println!("[*]\t{}\t{}", task.id, task.content),
            Status::DONE => println!("[Y]\t{}\t{}", task.id, task.status),
            _ => println!("[ELSE]\t{}\t{}", task.id, task.status)
        }
    }
}