use crate::{Communicator, Content, Conversation, FunctionsRegistry, Message};

pub struct FunctionExecutorAgent {
    pub registry: Option<FunctionsRegistry>
}

impl FunctionExecutorAgent {
    pub fn new() -> Self {
        let registry = None;
        Self { registry }
    }
}

#[async_trait::async_trait(?Send)]
impl Communicator for FunctionExecutorAgent {
    fn name(&self) -> &str {
        "Function Executor"
    }

    async fn receive(&mut self, sender: &mut dyn Communicator, conversation: &mut Conversation) -> Option<Message> {
        if let Content::FunctionCall(function_call) = &conversation.last_message().unwrap().content {
            if let Some(result) = self.registry.as_ref().and_then(|registry| registry.call(&function_call)) {
                self.send(sender, conversation, result.into()).await
            } else {
                None
            }
        } else {
            None
        }
    }
}