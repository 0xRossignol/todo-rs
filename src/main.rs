mod task;
mod run;

use crate::task::Database;
use std::{env, process};

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否有足够的参数
    if args.len() < 2 {
        print_usage_and_exit();
    }

    let command = &args[1];

    let db = Database::open("rodo_tasks.csv").unwrap_or_else(|err| {
        eprintln!("Error opening database: {}", err);
        process::exit(1);
    });

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                print_usage_and_exit();
            }
            run::add(&args, db)
        }
        "rm" => {
            if args.len() < 3 {
                print_usage_and_exit();
            }
            run::rm(&args, db)
        }
        "ls" => {
            run::ls(db)
        }

        _ => {
            eprintln!("Unknown command: {}\n", command);
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() -> ! {
    eprintln!("Usage: rodo [command] [options]");
    process::exit(1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn cli_process() {}
}
