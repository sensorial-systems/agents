pub mod models;
mod instruction;

pub use instruction::*;

use crate::{models::GPT4, Agent, Communicator, Content, Conversation, Message};
use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap)]
pub struct AutoAgent {
    #[shrinkwrap(main_field)]
    agent: Agent,
    model: GPT4,
    instruction: Instruction,
}

impl AutoAgent {
    pub fn new(model: impl AsRef<GPT4>, name: impl Into<String>) -> Self {
        let model = model.as_ref().clone();
        let agent = Agent::new(name);
        let instruction = Default::default();

        Self { model, agent, instruction }
    }

    pub fn with_notifications(mut self, notifications: Option<impl Fn(&mut Conversation) + 'static>) -> Self {
        self.agent = self.agent.with_notifications(notifications);
        self
    }

    pub fn with_instruction(mut self, instruction: impl Into<Instruction>) -> Self {
        self.instruction = instruction.into();
        self
    }
}

impl AutoAgent {
    pub async fn talk_to(&mut self, recipient: &mut dyn Communicator) {
        let mut conversation = Conversation::new();
        self.receive(recipient, &mut conversation).await;
    }
}

#[async_trait::async_trait(?Send)]
impl Communicator for AutoAgent {
    fn name(&self) -> &str {
        self.agent.name()
    }

    async fn send(&mut self, recipient: &mut dyn Communicator, conversation: &mut Conversation, content: Content) {
        conversation.add_message(Message::new(self, recipient, content));
        recipient.receive(self, conversation).await;
    }

    async fn receive(&mut self, sender: &mut dyn Communicator, conversation: &mut Conversation) {
        if let Some(notifications) = &self.notifications {
            notifications(conversation);
        }
        if !conversation.has_terminated() {
            let content = self.model.complete(&self.instruction, conversation).await;
            match &content {
                Content::FunctionCall(function_call) => {
                    conversation.add_message(Message::new(self, self, content.clone()));
                    if let Some(result) = self.instruction.functions.call(&function_call) {
                        conversation.add_message(Message::new(self, self, result));
                        let content = self.model.complete(&self.instruction, conversation).await;
                        self.send(sender, conversation, content.into()).await;
                    }
                }
                Content::Text(_) => {
                    self.send(sender, conversation, content.into()).await;
                }
            }
        }
    }
}
