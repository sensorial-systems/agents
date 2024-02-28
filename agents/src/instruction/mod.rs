use openai::chat::{ChatCompletionFunctionDefinition, ChatCompletionMessage, ChatCompletionMessageRole};

mod functions;
pub use functions::*;

mod multicall;
pub use multicall::*;

use crate::AgentFunction;

#[derive(Default)]
pub struct Instruction {
    pub message: String,
    pub functions: FunctionsRegistry,
}

impl Instruction {
    pub fn new(message: impl Into<String>) -> Self {
        let message = message.into();
        let functions = Default::default();
        Self { message, functions }
    }

    pub fn with_functions<Function: Into<AgentFunction>>(mut self, function: impl IntoIterator<Item = Function>) -> Self {
        self.functions.registry.extend(function.into_iter().map(|f| f.into()));
        self
    }

    pub fn message(&self) -> ChatCompletionMessage {
        self.into()
    }

    pub fn functions(&self) -> Vec<openai::chat::ChatCompletionFunctionDefinition> {
        self.into()
    }
}

impl From<String> for Instruction {
    fn from(message: String) -> Self {
        Instruction::new(message)
    }
}

impl From<&String> for Instruction {
    fn from(message: &String) -> Self {
        Instruction::new(message)
    }
}

impl From<&str> for Instruction {
    fn from(message: &str) -> Self {
        Instruction::new(message)
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

