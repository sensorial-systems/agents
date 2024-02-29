use openai::chat::{ChatCompletion, ChatCompletionFunctionDefinition, ChatCompletionMessage, ChatCompletionMessageRole};

use crate::{Content, Conversation, Instruction};


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

    pub async fn complete(&self, instruction: &Instruction, conversation: &Conversation) -> Content {
        openai::set_key(self.api_key.clone());
        let messages = std::iter::once(ChatCompletionMessage::from(instruction))
            .chain(conversation.history().iter().cloned().map(|x| x.into()))
            .collect::<Vec<_>>();
        let chat_completion = ChatCompletion::builder(self.name(), messages)
            .functions(Vec::from(instruction))
            .temperature(0.0)
            .create()
            .await
            .unwrap();
        let content = chat_completion.choices.first().unwrap().message.clone();
        content.into()
    }
}

impl AsRef<GPT4> for GPT4 {
    fn as_ref(&self) -> &GPT4 {
        self
    }
}

impl From<&Instruction> for ChatCompletionMessage {
    fn from(instruction: &Instruction) -> Self {
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some(instruction.message.clone()),
            name: None,
            function_call: None
        }
    }
}

impl From<&Instruction> for Vec<openai::chat::ChatCompletionFunctionDefinition> {
    fn from(instruction: &Instruction) -> Self {
        instruction.functions.iter().map(|f| {
            ChatCompletionFunctionDefinition {
                name: f.name.clone(),
                description: Some(f.description.clone()),
                parameters: Some(f.parameters.clone())
            }
        }).collect()
    }
}
