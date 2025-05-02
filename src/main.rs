use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    let mut history: Vec<String> = Vec::new();

    loop {
        print!("{}> ", env::current_dir().unwrap().display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        history.push(input.to_string());
        run_command(input, &history);
    }
}

fn run_command(input: &str, history: &[String]) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let cmd = parts[0];
    let args = &parts[1..];

    match cmd {
        "exit" => std::process::exit(0),

        "cd" => {
            if let Some(dir) = args.first() {
                if let Err(e) = env::set_current_dir(dir) {
                    eprintln!("cd: {}", e);
                }
            } else {
                // If no directory is specified, change to home directory
                if let Ok(home) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
                    if let Err(e) = env::set_current_dir(&home) {
                        eprintln!("cd: Failed to change to home directory: {}", e);
                    }
                } else {
                    eprintln!("cd: Home directory not found");
                }
            }
        }

        "cls" => {
            // Clear screen for Windows and ANSI terminals
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            io::stdout().flush().unwrap();
        }

        "clear" => {
            // Clear screen for Windows and ANSI terminals (alias for cls)
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            io::stdout().flush().unwrap();
        }

        "ls" | "dir" => {
            let path = args.first().unwrap_or(&".");
            match fs::read_dir(path) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        let metadata = match entry.metadata() {
                            Ok(meta) => meta,
                            Err(_) => continue,
                        };
                        let file_type = if metadata.is_dir() { "DIR" } else { "FILE" };
                        let size = if metadata.is_file() { metadata.len() } else { 0 };
                        println!("{:4} {:8} {}", file_type, size, entry.file_name().to_string_lossy());
                    }
                }
                Err(e) => eprintln!("ls: {}", e),
            }
        }

        "pwd" => {
            println!("{}", env::current_dir().unwrap().display());
        }

        "history" => {
            for (i, cmd) in history.iter().enumerate() {
                println!("{}: {}", i + 1, cmd);
            }
        }

        "touch" => {
            if let Some(filename) = args.first() {
                if let Err(e) = fs::File::create(filename) {
                    eprintln!("touch: {}", e);
                }
            } else {
                eprintln!("touch: missing file operand");
            }
        }

        "mkdir" => {
            if let Some(dirname) = args.first() {
                if let Err(e) = fs::create_dir_all(dirname) {
                    eprintln!("mkdir: {}", e);
                }
            } else {
                eprintln!("mkdir: missing directory operand");
            }
        }

        "cat" => {
            if let Some(filename) = args.first() {
                match fs::read_to_string(filename) {
                    Ok(content) => println!("{}", content),
                    Err(e) => eprintln!("cat: {}", e),
                }
            } else {
                eprintln!("cat: missing file operand");
            }
        }

        "rm" => {
            if let Some(path) = args.first() {
                let path = Path::new(path);
                if path.is_dir() {
                    if let Err(e) = fs::remove_dir_all(path) {
                        eprintln!("rm: {}", e);
                    }
                } else {
                    if let Err(e) = fs::remove_file(path) {
                        eprintln!("rm: {}", e);
                    }
                }
            } else {
                eprintln!("rm: missing operand");
            }
        }

        "echo" => {
            println!("{}", args.join(" "));
        }

        _ => {
            match Command::new(cmd)
                .args(args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
            {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("Command exited with code: {}", status.code().unwrap_or(-1));
                    }
                }
                Err(_) => {
                    eprintln!("Command '{}' not found", cmd);
                }
            }
        }
    }
}