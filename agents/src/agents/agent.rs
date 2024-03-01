use std::pin::Pin;

use crate::{Communicator, Content, Conversation, Message};

pub struct Agent {
    name: String,
    pub notifications: Option<Pin<Box<dyn Fn(&mut Conversation)>>>
}

impl Agent {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let notifications = Default::default();
        Self { name, notifications }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn with_notifications(mut self, notifications: Option<impl Fn(&mut Conversation) + 'static>) -> Self {
        self.notifications = notifications.map(|x| Box::pin(x) as Pin<Box<dyn Fn(&mut Conversation)>>);
        self
    }
}

#[async_trait::async_trait(?Send)]
impl Communicator for Agent {
    fn name(&self) -> &str {
        self.name()
    }

    async fn send(&mut self, _recipient: &mut dyn Communicator, _conversation: &mut Conversation, _content: Content) -> Option<Message> {
        None
    }

    async fn receive(&mut self, _sender: &mut dyn Communicator, _conversation: &mut Conversation) -> Option<Message> {
        None
    }
}
