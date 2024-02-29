use crate::{AgentFunction, FunctionCall};
#[derive(Clone, Default)]
pub struct FunctionsRegistry {
    pub registry: Vec<AgentFunction>
}

impl FunctionsRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, function: AgentFunction) {
        self.registry.push(function);
    }

    pub fn retain(&mut self, f: impl Fn(&AgentFunction) -> bool) {
        self.registry.retain(f);
    }

    pub fn iter(&self) -> std::slice::Iter<AgentFunction> {
        self.registry.iter()
    }

    pub fn call(&self, function_call: &FunctionCall) -> Option<String> {
        self
            .registry
            .iter()
            .find(|x| x.name == function_call.name)
            .map(|function|
                function.call(self, function_call.arguments.clone())
            )
    }
}

impl From<Vec<AgentFunction>> for FunctionsRegistry {
    fn from(registry: Vec<AgentFunction>) -> Self {
        Self { registry }
    }
}