mod controller;
use controller::email_input::EmailInput;

mod use_case;
use use_case::send_email::{EmailDTO, SimpleSendEmail, SendEmail};

mod domain;
use domain::user_agent::SimpleUserAgent;

fn main() {
    let email_input = EmailInput::new();  
    let mut send_email = SimpleSendEmail::new(
        Box::new(SimpleUserAgent {
            stream: None,
            buffer: [0;512]
        }),
        EmailDTO {
            client_address: email_input.client_address,
            from: email_input.from,
            to: email_input.to,
            data: email_input.data,
        }
    );
    send_email.send();
}

