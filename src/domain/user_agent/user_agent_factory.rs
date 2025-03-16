use super::user_agent::UserAgent;
use super::simple_user_agent::SimpleUserAgent;

pub struct UserAgentFactory;

impl UserAgentFactory {
    pub fn create_simple_user_agent() -> Box<dyn UserAgent>{
        Box::new(SimpleUserAgent {
            stream: None,
            buffer: [0;512]
        })
    }
}