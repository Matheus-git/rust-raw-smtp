use super::user_agent_factory::UserAgentFactory;
use super::user_agent::UserAgent;

pub struct UserAgentRepository;

impl UserAgentRepository {
    pub fn create_simple_user_agent() -> Box<dyn UserAgent>{
        UserAgentFactory::create_simple_user_agent()
    }
}