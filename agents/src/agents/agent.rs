use std::pin::Pin;

use crate::Conversation;

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
