use std::io::{self, Write};
#[derive(Debug)]
pub struct EmailInput {
    pub client_address: String,
    pub from: String,
    pub to: String,
    pub data: String,
}

impl EmailInput {
    pub fn new() -> Self {
        let default_client_address = "127.0.0.1:1025".to_string();
        let default_from = "sender@example.com".to_string();
        let default_to = "recipient@example.com".to_string();
        let default_data = "This is a test email.".to_string();

        let client_address = read_input(
            &format!("Enter the SMTP client address (default: {}): ", default_client_address),
            default_client_address,
        );
        let from = read_input(
            &format!("Enter the sender's address (default: {}): ", default_from),
            default_from,
        );
        let to = read_input(
            &format!("Enter the recipient's address (default: {}): ", default_to),
            default_to,
        );
        let data = read_input(
            &format!("Enter the email body (default: {}): ", default_data),
            default_data,
        );

        EmailInput {
            client_address,
            from,
            to,
            data,
        }
    }
}

fn read_input(prompt: &str, default: String) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let input = input.trim();
    if input.is_empty() {
        default
    } else {
        input.to_string()
    }
}