pub mod models;
mod instruction;

pub use instruction::*;

use crate::{models::GPT4, Agent, Communicator, Conversation, FunctionExecutorAgent, Message};
use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap)]
pub struct AutoAgent {
    #[shrinkwrap(main_field)]
    agent: Agent,
    executor: FunctionExecutorAgent,
    model: GPT4,
    instruction: Instruction,
}

impl AutoAgent {
    pub fn new(model: impl AsRef<GPT4>, name: impl Into<String>) -> Self {
        let model = model.as_ref().clone();
        let agent = Agent::new(name);
        let instruction = Instruction::default();
        let executor = FunctionExecutorAgent::new();

        Self { model, agent, executor, instruction }
    }

    pub fn with_notifications(mut self, notifications: Option<impl Fn(&mut Conversation) + 'static>) -> Self {
        self.agent = self.agent.with_notifications(notifications);
        self
    }

    pub fn with_instruction(mut self, instruction: impl Into<Instruction>) -> Self {
        self.instruction = instruction.into();
        // FIXME: Instruction should contain the JsonSchema of the functions, but executor should know about the callbacks. How can we separate this?
        self.executor.registry = Some(self.instruction.functions.clone());
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

    async fn receive(&mut self, sender: &mut dyn Communicator, conversation: &mut Conversation) -> Option<Message> {
        if let Some(notifications) = &self.notifications {
            notifications(conversation);
        }
        if !conversation.has_terminated() {
            let content = self.model.complete(&self.instruction, conversation).await;
            if content.is_function_call() {
                // FIXME: This is a hack to avoid the borrow checker. We should find a better way to do this.
                let executor = unsafe { &mut *(&mut self.executor as *mut FunctionExecutorAgent) }; 
                if let Some(message) = self.send(executor, conversation, content.into()).await {
                    self.send(sender, conversation, message.content).await
                } else {
                    None
                }
            } else {
                self.send(sender, conversation, content.into()).await
            }
        } else {
            None
        }
    }
}
