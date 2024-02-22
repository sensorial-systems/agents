use openai::chat::ChatCompletion;

use crate::{Conversation, Instruction, Message};


#[derive(Clone)]
pub struct GPT4 {
    pub api_key: String
}

impl GPT4 {
    pub fn new(api_key: impl Into<String>) -> Self {
        let api_key = api_key.into();
        Self { api_key }
    }

    pub fn name(&self) -> &str {
        "gpt-4"
    }

    pub async fn complete<'a>(&'a self, instruction: &Instruction, conversation: &'a mut Conversation) -> &'a mut Message {
        openai::set_key(self.api_key.clone());
        let messages = std::iter::once(instruction.message())
            .chain(conversation.history().iter().cloned().map(|x| x.into()))
            .collect::<Vec<_>>();
        let chat_completion = ChatCompletion::builder(self.name(), messages)
            .functions(instruction.functions())
            .temperature(0.0)
            .create()
            .await
            .unwrap();
        let message = chat_completion.choices.first().unwrap().message.clone();
        conversation.history_mut().push(message.into());
        conversation.history_mut().last_mut().unwrap()
    }
}

impl AsRef<GPT4> for GPT4 {
    fn as_ref(&self) -> &GPT4 {
        self
    }
}