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

    let mut db = Database::open(".rododb").unwrap_or_else(|err| {
        eprintln!("Error opening database: {}", err);
        process::exit(1);
    });

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                print_usage_and_exit();
            }

            let contents = &args[2..].join(" ");
            let id = 1;
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
            println!("Listing all items...");
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
