mod controller;
use controller::email_input::EmailInput;

mod use_case;
use use_case::send_email::{EmailDTO, SimpleSendEmail, SendEmail};

mod domain;
use domain::user_agent::user_agent_repository::UserAgentRepository;

fn main() {
    let email_input = EmailInput::new();  
    let email_dto = EmailDTO {
        from: email_input.from,
        to: email_input.to,
        subject: email_input.subject,
        data: email_input.data,
    };
    
    let user_agent = UserAgentRepository::create_simple_user_agent();

    let mut send_email = SimpleSendEmail::new(user_agent, email_dto);
    send_email.send();

    println!("\nEmail sent successfully!\n")
}
