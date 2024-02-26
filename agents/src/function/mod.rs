use schemars::{schema_for, JsonSchema};
use serde::{de::DeserializeOwned, Serialize};
use derivative::Derivative;

#[derive(Derivative, Serialize)]
#[derivative(Debug, PartialEq)]
pub struct AgentFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    #[serde(skip)]
    #[derivative(Debug="ignore", PartialEq="ignore")]
    pub callback: Box<dyn Fn(String) -> String>
}

impl AgentFunction {
    pub fn new<Parameter: JsonSchema + DeserializeOwned>(name: impl Into<String>, callback: impl Fn(Parameter) -> String + 'static) -> Self {
        let name = name.into();
        let description = Default::default();
        let parameters = serde_json::to_value(schema_for!(Parameter)).unwrap();
        let callback = move |arguments: String| {
            let arguments = serde_json::from_str::<Parameter>(&arguments).unwrap();
            callback(arguments)
        };
        let callback = Box::new(callback);

        Self { name, description, parameters, callback }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}
