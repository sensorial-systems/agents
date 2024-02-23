use enum_as_inner::EnumAsInner;
use openai::chat::ChatCompletionFunctionCall;

#[derive(Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String
}

#[derive(Clone, EnumAsInner)]
pub enum Content {
    Text(String),
    FunctionCall(FunctionCall)
}

impl From<ChatCompletionFunctionCall> for FunctionCall {
    fn from(call: ChatCompletionFunctionCall) -> Self {
        let name = call.name;
        let arguments = call.arguments;
        Self { name, arguments }
    }
}

impl From<FunctionCall> for ChatCompletionFunctionCall {
    fn from(call: FunctionCall) -> Self {
        let name = call.name;
        let arguments = call.arguments;
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