mod content;
pub use content::*;

use openai::chat::{ChatCompletionMessage, ChatCompletionMessageRole};
use crate::Communicator;

#[derive(Clone)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub content: Content,
}

impl Message {
    pub fn new(from: &dyn Communicator, to: &dyn Communicator, content: impl Into<Content>) -> Self {
        let from = from.name().into();
        let to = to.name().into();
        let content = content.into();
        Self { from, to, content }
    }

    pub fn sign(&mut self, from: &dyn Communicator, to: &dyn Communicator) {
        self.from = from.name().into();
        self.to = to.name().into();
    }
}

impl From<&str> for Content {
    fn from(content: &str) -> Self {
        Content::Text(content.into())
    }
}

impl From<String> for Content {
    fn from(content: String) -> Self {
        Content::Text(content)
    }
}

impl From<&str> for Message {
    fn from(content: &str) -> Self {
        content.to_string().into()
    }
}

impl From<String> for Message {
    fn from(content: String) -> Self {
        let from = Default::default();
        let to = Default::default();
        let content = content.into();
        Message { content, from, to }
    }
}

impl From<Content> for Message {
    fn from(content: Content) -> Self {
        let from = Default::default();
        let to = Default::default();
        Message { content, from, to }
    }
}

impl From<Message> for ChatCompletionMessage {
    fn from(message: Message) -> Self {
        let (role, content, function_call) = match message.content {
            Content::Text(content) => (ChatCompletionMessageRole::User, Some(content), None),
            Content::FunctionCall(call) => (ChatCompletionMessageRole::Assistant, None, Some(call.into()))
        };
        let name = None; // TODO: Add support for name.
        ChatCompletionMessage { name, role, content, function_call }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} (to {}):\n{}\n", self.from, self.to, self.content)
    }
}