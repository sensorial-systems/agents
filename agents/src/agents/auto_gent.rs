use crate::{models::GPT4, Agent, Communication, Content, Conversation, Instruction, Message};
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
    pub async fn talk_to(&mut self, recipient: &mut Self) {
        let mut conversation = Conversation::new();
        self.receive(recipient, &mut conversation).await;
    }
}

#[async_trait::async_trait(?Send)]
impl Communication for AutoAgent {
    async fn send(&mut self, recipient: &mut Self, conversation: &mut Conversation, message: impl Into<Message>) {
        let mut message = message.into();
        message.sign(self, recipient);
        conversation.add_message(message);
        recipient.receive(self, conversation).await;
    }

    async fn receive(&mut self, sender: &mut Self, conversation: &mut Conversation) {
        if let Some(notifications) = &self.notifications {
            notifications(conversation);
        }
        if !conversation.has_terminated() {
            // FIXME: This message isn't being added in the conversation.
            let message = self.model.complete(&self.instruction, conversation).await;
            match message.content {
                Content::FunctionCall(function_call) => {
                    if let Some(result) = self.instruction.functions.call(&function_call) {
                        let mut message = Message::from(result);
                        message.sign(self, self);
                        conversation.add_message(message);
                        let message = self.model.complete(&self.instruction, conversation).await;
                        self.send(sender, conversation, message).await;
                    }
                }
                Content::Text(_) => {
                    self.send(sender, conversation, message).await;
                }
            }
        }
    }
}
