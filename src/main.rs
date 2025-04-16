use std::env;
use std::fs;
use std::io::{self, Write};
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
        run_command(input);
    }
}

fn run_command(input: &str) {
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
                eprintln!("cd: missing operand");
            }
        }

        "cls" => {
            // Clear screen for Windows and ANSI terminals
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            io::stdout().flush().unwrap();
        }

        "ls" => {
            let path = args.first().unwrap_or(&".");
            match fs::read_dir(path) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        println!("{}", entry.file_name().to_string_lossy());
                    }
                }
                Err(e) => eprintln!("ls: {}", e),
            }
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
