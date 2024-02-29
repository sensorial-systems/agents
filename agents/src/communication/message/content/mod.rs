use enum_as_inner::EnumAsInner;
use openai::chat::{ChatCompletionFunctionCall, ChatCompletionMessage};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
/// A function call
pub struct FunctionCall {
    /// The name of the function
    pub name: String,
    /// The arguments to the function
    pub arguments: serde_json::Value
}

#[derive(Clone, EnumAsInner)]
pub enum Content {
    Text(String),
    FunctionCall(FunctionCall)
}

impl From<ChatCompletionFunctionCall> for FunctionCall {
    fn from(call: ChatCompletionFunctionCall) -> Self {
        let name = call.name;
        let arguments = serde_json::from_str(&call.arguments).unwrap();
        Self { name, arguments }
    }
}

impl From<FunctionCall> for ChatCompletionFunctionCall {
    fn from(call: FunctionCall) -> Self {
        let name = call.name;
        let arguments = serde_json::to_string(&call.arguments).unwrap();
        Self { name, arguments }
    }
}

impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Content::Text(text) => write!(f, "{}", text),
            Content::FunctionCall(call) => write!(f, "{}: {}", call.name, call.arguments)
        }
    }
}

impl From<ChatCompletionMessage> for Content {
    fn from(message: ChatCompletionMessage) -> Self {
        if let Some(content) = message.content {
            Content::Text(content)
        } else if let Some(function_call) = message.function_call {
            Content::FunctionCall(function_call.into())
        } else {
            Content::Text(Default::default())
        }
    }
}