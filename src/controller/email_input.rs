use std::io::{self, Write};

pub struct EmailInput {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub data: String,
}

impl EmailInput {
    pub fn new() -> Self {
        let from = read_input("Enter the sender's address:");
        let to = read_input("Enter the recipient's address:");
        let subject = read_input("Enter the subject:");
        let data = read_input("Enter the email body:");

        EmailInput {
            from,
            to,
            subject,
            data,
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("\x1b[1m{} \x1b[0m", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let input = input.trim();
    input.to_string()
}
