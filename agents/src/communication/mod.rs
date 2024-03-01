mod conversation;
mod message;

pub use conversation::*;
pub use message::*;

#[async_trait::async_trait(?Send)]
pub trait Communicator {
    fn name(&self) -> &str;
    async fn send(&mut self, recipient: &mut dyn Communicator, conversation: &mut Conversation, content: Content) -> Option<Message>;
    async fn receive(&mut self, sender: &mut dyn Communicator, conversation: &mut Conversation) -> Option<Message>;
}

impl From<&dyn Communicator> for String {
    fn from(communicator: &dyn Communicator) -> Self {
        communicator.name().into()
    }
}
