use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use yew::worker::{Agent, AgentLink, Context, HandlerId};

#[derive(Debug, Deserialize, Serialize)]
pub enum TokenRequest {
    UpdateToken(String),
    GetToken,
}

pub struct TokenAgent {
    link: AgentLink<TokenAgent>,
    subscribers: HashSet<HandlerId>,
    token: Option<String>,
}

impl Agent for TokenAgent {
    type Reach = Context;
    type Message = ();
    type Input = TokenRequest;
    type Output = String;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
            token: Default::default(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            Self::Input::UpdateToken(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, s.clone());
                }
                self.token = Some(s);
            }
            Self::Input::GetToken => {
                if let Some(token) = &self.token {
                    self.link.respond(id, token.clone());
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
