use crate::{Conversation, Message};

#[async_trait::async_trait(?Send)]
pub trait Communication {
    async fn send(&mut self, recipient: &mut Self, conversation: &mut Conversation, message: impl Into<Message>);
    async fn receive(&mut self, sender: &mut Self, conversation: &mut Conversation);
}