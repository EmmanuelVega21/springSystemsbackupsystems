use std::io::{self, Write};
use std::process::Command;

// Each variant holds the data it needs — no extra variables required
enum FileOperation {
    List(String),           // directory path
    Display(String),        // file path
    Create(String, String), // file path + content
    Remove(String),         // file path
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            Command::new("ls")
                .arg(&path)
                .status()
                .expect("Failed to execute ls");
        }

        FileOperation::Display(path) => {
            Command::new("cat")
                .arg(&path)
                .status()
                .expect("Failed to execute cat");
        }

        FileOperation::Create(path, content) => {
      
            let cmd = format!("echo '{}' > {}", content, path);
            let result = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .status()
                .expect("Failed to execute sh");

            if result.success() {
                println!("File '{}' created successfully.", path);
            } else {
                eprintln!("Failed to create file '{}'.", path);
            }
        }

        FileOperation::Remove(path) => {
            let result = Command::new("rm")
                .arg(&path)
                .status()
                .expect("Failed to execute rm");

            if result.success() {
                println!("File '{}' removed successfully.", path);
            } else {
                eprintln!("Failed to remove file '{}'.", path);
            }
        }

        FileOperation::Pwd => {
            Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");
        }
    }
}

// Helper: print the menu
fn print_menu() {
    println!("\nFile Operations Menu:");
    println!("1. List files in a directory");
    println!("2. Display file contents");
    println!("3. Create a new file");
    println!("4. Remove a file");
    println!("5. Print working directory");
    println!("0. Exit");
    print!("\nEnter your choice (0-5): ");
    io::stdout().flush().unwrap(); // ensure prompt appears before input
}

// Helper: read a line of input and trim whitespace
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Helper: prompt with a label and return the trimmed input
fn prompt(label: &str) -> String {
    print!("{}", label);
    io::stdout().flush().unwrap();
    read_line()
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        print_menu();

        let choice = read_line();

        // Map the menu number to a FileOperation (or None for invalid)
        let operation: Option<FileOperation> = match choice.as_str() {
            "0" => {
                println!("\nGoodbye!");
                break; // exit the loop and end the program
            }
            "1" => {
                let path = prompt("Enter directory path: ");
                Some(FileOperation::List(path))
            }
            "2" => {
                let path = prompt("Enter file path: ");
                Some(FileOperation::Display(path))
            }
            "3" => {
                let path    = prompt("Enter file path: ");
                let content = prompt("Enter content: ");
                Some(FileOperation::Create(path, content))
            }
            "4" => {
                let path = prompt("Enter file path: ");
                Some(FileOperation::Remove(path))
            }
            "5" => Some(FileOperation::Pwd),
            _ => {
                println!("Invalid option '{}'. Please enter 0-5.", choice);
                None
            }
        };

        // Only call perform_operation if we got a valid variant
        if let Some(op) = operation {
            perform_operation(op);
        }
    }
}