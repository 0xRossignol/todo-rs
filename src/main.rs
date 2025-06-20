mod task;

use crate::task::{Database, Status};
use std::{env, process};

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否有足够的参数
    if args.len() < 2 {
        print_usage_and_exit();
    }

    let command = &args[1];

    let mut db = Database::open("rodo_tasks.csv").unwrap_or_else(|err| {
        eprintln!("Error opening database: {}", err);
        process::exit(1);
    });

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                print_usage_and_exit();
            }

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
        "rm" => {
            if args.len() < 3 {
                print_usage_and_exit();
            }

            println!("Removing: {}", args[2]);
        }
        "ls" => {
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

        _ => {
            eprintln!("Unknown command: {}\n", command);
            process::exit(1);
        }
    }
}

fn print_usage_and_exit() -> ! {
    eprintln!("Usage: rodo [command] [options]");
    process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_process() {}
}
