mod controller;
use controller::email_input::EmailInput;

mod use_case;
use use_case::send_email::{EmailDTO, SimpleSendEmail};

fn main() {
    let email_input = EmailInput::new();  
    let send_email = SimpleSendEmail::new(
        user_agent,
        EmailDTO {
            client_address: email_input.client_address,
            from: email_input.from,
            to: email_input.to,
            data: email_input.data,
        }
    );
}