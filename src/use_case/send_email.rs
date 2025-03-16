use crate::domain::user_agent::UserAgent;

pub trait SendEmail {
    fn send(&mut self) {
        self.conn();
        self.hello();
        self.from();
        self.to();
        self.data();
        self.quit();
    }

    fn conn(&mut self);
    fn hello(&mut self);
    fn from(&mut self);
    fn to(&mut self);
    fn data(&mut self);
    fn quit(&mut self);
}

pub struct EmailDTO {
    pub client_address: String,
    pub from: String,
    pub to: String,
    pub data: String,
}

pub struct SimpleSendEmail {
    user_agent: Box<dyn UserAgent>,
    email: EmailDTO
}

impl SimpleSendEmail {
    pub fn new(user_agent: Box<dyn UserAgent>, email: EmailDTO) -> Self {
        SimpleSendEmail {
            user_agent,
            email
        }
    }
}

impl SendEmail for SimpleSendEmail {
    fn conn(&mut self) {
        self.user_agent.conn();
    }
    fn hello(&mut self) {
        self.user_agent.hello();
    }
    fn from(&mut self) {
        let from = self.email.from.clone();
        self.user_agent.from(from);
    }
    fn to(&mut self) {
        let to = self.email.to.clone();
        self.user_agent.to(to);
    }
    fn data(&mut self) {
        let from = self.email.from.clone();
        let to = self.email.to.clone();
        let data = self.email.data.clone();
        let email_body = format!(
            "From: {}\r\n\
            To: {}\r\n\
            Subject: Test Email\r\n\
            \r\n\
            {}\r\n\
            .\r\n", from, to, data);
        self.user_agent.data(email_body);
    }
    fn quit(&mut self) {
        self.user_agent.quit();
    }
}
