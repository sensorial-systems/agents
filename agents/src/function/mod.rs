use std::rc::Rc;

use schemars::{gen::SchemaSettings, JsonSchema};
use serde::{de::DeserializeOwned, Serialize};
use derivative::Derivative;

use crate::FunctionsRegistry;

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug, PartialEq)]
pub struct AgentFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    #[serde(skip)]
    #[derivative(Debug="ignore", PartialEq="ignore")]
    pub callback: Rc<dyn Fn(&FunctionsRegistry, serde_json::Value) -> String + Send + Sync>
}

impl AgentFunction {
    pub fn new<Parameter: JsonSchema + DeserializeOwned>(name: impl Into<String>, callback: impl Fn(&FunctionsRegistry, Parameter) -> String + 'static + Send + Sync) -> Self {
        let name = name.into();
        let description = Default::default();

        let settings = SchemaSettings::draft07().with(|s| {
            s.inline_subschemas = true;
        });
        let gen = settings.into_generator();
        let schema = gen.into_root_schema_for::<Parameter>();
        let parameters = serde_json::to_value(schema).unwrap();


        let callback = move |registry: &FunctionsRegistry, arguments: serde_json::Value| {
            let arguments = serde_json::from_value::<Parameter>(arguments).unwrap();
            callback(registry, arguments)
        };
        let callback = Rc::new(callback);

        Self { name, description, parameters, callback }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn call(&self, registry: &FunctionsRegistry, arguments: serde_json::Value) -> String {
        (self.callback)(registry, arguments)
    }
}
