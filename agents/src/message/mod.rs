use openai::chat::{ChatCompletionMessage, ChatCompletionMessageRole};

#[derive(Clone)]
pub struct Message {
    pub content: String,
}

impl From<&str> for Message {
    fn from(content: &str) -> Self {
        let content = content.into();
        Message { content }
    }
}

impl From<ChatCompletionMessage> for Message {
    fn from(message: ChatCompletionMessage) -> Self {
        let content = message.content.unwrap();
        Message { content }
    }
}

impl From<Message> for ChatCompletionMessage {
    fn from(message: Message) -> Self {
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(message.content),
            name: None,
            function_call: None
        }
    }
}
