use std::{env, process};

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否有足够的参数
    if args.len() < 2 {
        eprintln!("Usage: rodo [command] [options]");
        process::exit(1);
    }

    let command = &args[1];
    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: rodo add [contents]\n");
                process::exit(1);
            }

            println!("Adding: {}", args[2]);
        }
        "rm" => {
            if args.len() < 3 {
                eprintln!("Usage: rodo rm [contents]\n");
                process::exit(1);
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
