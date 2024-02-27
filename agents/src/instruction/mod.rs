use openai::chat::{ChatCompletionFunctionDefinition, ChatCompletionMessage, ChatCompletionMessageRole};

use crate::AgentFunction;

mod multicall;
use multicall::MultiCallParameters;

#[derive(Default)]
pub struct Instruction {
    pub message: String,
    pub functions: Vec<AgentFunction>
}

impl Instruction {
    pub fn new(message: impl Into<String>) -> Self {
        let message = message.into();
        let functions = Default::default();
        Self { message, functions }
    }

    pub fn with_multicall(mut self, allow: bool) -> Self {
        if allow {
            let functions = self.functions.clone(); // FIXME: If with_multicall is called before with_functions, this will be empty.
            self.functions.push(
                AgentFunction::new("multicall", move |parameters: MultiCallParameters| {
                    let mut output = Vec::new();
                    for call in parameters.calls {
                        if let Some(function) = functions.iter().find(|f| f.name == call.name) {
                            output.push((function.callback)(call.arguments));
                        }
                    }
                    output.join(", ")
                }).with_description("Call multiple functions at once.")
            );
        } else {
            self.functions.retain(|f| f.name != "multicall");
        }
        self
    }

    pub fn with_functions(mut self, function: impl Into<Vec<AgentFunction>>) -> Self {
        self.functions = function.into();
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

