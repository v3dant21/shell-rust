# shell-rust
# Shell in Rust

![Shell Interface](images/shell.png)

This project implements a simple custom shell written in Rust. The goal of this project is to demonstrate how to build a basic command-line interpreter (shell) from scratch without relying on external libraries or dependencies.

## Features

- Basic command execution (e.g., running commands from the terminal).
- Support for built-in commands like `exit`, `cd`, and `help`.
- Ability to handle both relative and absolute file paths.
- Supports piping and redirection (optional, based on further implementation).

## Getting Started

To get started with the **Shell in Rust** project, follow the steps below.

### Prerequisites

Before running this project, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/learn/get-started) - The programming language used for this project.

### Clone the Repository

```bash
git clone https://github.com/v3dant21/shell-rust.git
cd shell-rust
```

### Add the Example Screenshot

Create an `images/` directory in the project root, then copy your shell screenshot into it:

```bash
mkdir images
cp /path/to/your/shell.png images/
```  

### Build and Run

Compile and run the shell:

```bash
cargo build --release
./target/release/shell-rust
```

## Usage

Once running, you can type commands like you would in a normal terminal:

```text
> help
Available commands:
  - cd [dir]   Change directory
  - exit       Exit the shell
  - help       Show this help message
  - ls         List files (external command)
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.



