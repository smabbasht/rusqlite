use std::io::{self, stdout, Write};

fn main() {
    print_prompt();
    loop {
        print!("rusqlite> ");
        stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        if buffer.starts_with('.') {
            handle_meta_cmds(buffer);
        } else {
            handle_statements(buffer);
        }
    }
}

fn handle_meta_cmds(buffer: String) {}
fn handle_statements(buffer: String) {}

fn print_prompt() {
    println!("~ rusqlite");
    println!("RuSQLite version 0.1.0 2024-07-08 19:09:39");
    println!("Enter '.help' for usage hints.");
    println!("Connected to a transient in-memory database.");
    println!("Use '.open FILENAME' to reopen on a persistent database.");
}
