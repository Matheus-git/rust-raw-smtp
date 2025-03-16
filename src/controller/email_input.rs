use std::io::{self, Write};

#[derive(Debug)]
pub struct EmailInput {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub data: String,
}

impl EmailInput {
    pub fn new() -> Self {
        let default_from = "sender@example.com".to_string();
        let default_to = "recipient@example.com".to_string();
        let default_data = "This is a test email.".to_string();
        let default_subject = "Test Email".to_string();

        let from = read_input(
            &format!("Enter the sender's address (default: {}): ", default_from),
            default_from,
        );
        let to = read_input(
            &format!("Enter the recipient's address (default: {}): ", default_to),
            default_to,
        );
        let subject = read_input(
            &format!("Enter the subject (default: {}): ", default_subject),
            default_subject,
        );
        let data = read_input(
            &format!("Enter the email body (default: {}): ", default_data),
            default_data,
        );

        EmailInput {
            from,
            to,
            data,
            subject,
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
