mod conversation;
mod message;

pub use conversation::*;
pub use message::*;

#[async_trait::async_trait(?Send)]
pub trait Communicator {
    fn name(&self) -> &str;
    async fn send(&mut self, recipient: &mut dyn Communicator, conversation: &mut Conversation, content: Content) -> Option<Message>
    where Self: Sized
    {
        conversation.add_message(Message::new(self, recipient, content));
        recipient.receive(self, conversation).await
    }
    async fn receive(&mut self, _sender: &mut dyn Communicator, _conversation: &mut Conversation) -> Option<Message> {
        None
    }
}

impl From<&dyn Communicator> for String {
    fn from(communicator: &dyn Communicator) -> Self {
        communicator.name().into()
    }
}
