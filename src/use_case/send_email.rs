
trait SendEmail {
    fn send(&self) {
        self.data();
        self.to();
        self.data();
    }

    fn from(&self);
    fn to(&self);
    fn data(&self);
}

pub struct EmailDTO {
    pub client_address: String,
    pub from: String,
    pub to: String,
    pub data: String,
}

pub struct SimpleSendEmail {
    user_agent: UserAgent,
    email: EmailDTO
}

impl SimpleSendEmail {
    pub fn new(user_agent: UserAgent, email: EmailDTO) -> Self {
        SimpleSendEmail {
            user_agent,
            email
        }
    }
}

impl SendEmail for SimpleSendEmail {
    fn from(&self) {
        self.user_agent.from(self.email.from);
    }
    fn to(&self) {
        self.user_agent.to(self.email.to);
    }
    fn data(&self) {
        self.user_agent.data(self.email.data);
    }
}
