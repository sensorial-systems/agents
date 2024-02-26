use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use derivative::Derivative;

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug, PartialEq)]
pub struct AgentFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    #[serde(skip)]
    #[derivative(Debug="ignore", PartialEq="ignore")]
    pub callback: Option<Box<dyn Fn(String) -> String>>
}

impl AgentFunction {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let description = Default::default();
        let parameters = Default::default();
        let callback = None;
        Self { name, description, parameters, callback }
    }

    pub fn with_callback<Parameter: JsonSchema>(mut self, callback: impl Fn(String) -> String + 'static) -> Self {
        self.parameters = serde_json::to_value(schema_for!(Parameter)).unwrap();
        self.callback = Some(Box::new(callback));
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}
