use crate::{Conversation, Message};

#[async_trait::async_trait(?Send)]
pub trait Communicator {
    fn name(&self) -> &str;
    async fn send(&mut self, recipient: &mut dyn Communicator, conversation: &mut Conversation, message: Message);
    async fn receive(&mut self, sender: &mut dyn Communicator, conversation: &mut Conversation);
}