
mod parameter;
pub use parameter::*;

use serde::{ser::SerializeStruct, Deserialize, Serialize};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug, PartialEq)]
pub struct AgentFunction {
    pub name: String,
    pub description: String,
    pub parameters: Vec<FunctionParameter>,
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

    pub fn with_callback(mut self, callback: impl Fn(String) -> String + 'static) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_parameters(mut self, parameters: Vec<FunctionParameter>) -> Self {
        self.parameters = parameters;
        self
    }

}

impl Serialize for AgentFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("AgentFunction", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;

        #[derive(Serialize)]
        struct ParametersHelper<'a> {
            #[serde(rename = "type")]
            type_: &'a str,
            properties: std::collections::HashMap<&'a String, &'a FunctionParameter>
        }

        let parameters = ParametersHelper {
            type_: "object",
            properties: self.parameters.iter().map(|p| (&p.name, p)).collect()
        };
        state.serialize_field("parameters", &parameters)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for AgentFunction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct AgentFunctionHelper {
            name: String,
            description: String,
            parameters: std::collections::HashMap<String, FunctionParameter>,
        }

        let helper = AgentFunctionHelper::deserialize(deserializer)?;
        let name = helper.name;
        let description = helper.description;
        let parameters = helper.parameters.into_iter().map(|(name, mut v)| {
            v.name = name;
            v
        }).collect();
        let callback = None;
        Ok(Self { name, description, parameters, callback })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        let mut function = AgentFunction::new("quote_amount")
            .with_description("Quote the amount of money in a currency from another currency")
            .with_parameters(vec![
                FunctionParameter::number("amount").with_description("The amount of money to convert"),
                FunctionParameter::string("from").with_description("The currency of origin"),
                FunctionParameter::string("to").with_description("The currency of destination")
            ]);
        let serialized_function = serde_json::to_string_pretty(&function).unwrap();
        println!("{}", serialized_function);
        // let mut deserialized_function: AgentFunction = serde_json::from_str(&serialized_function).unwrap();
        // function.parameters.sort_by(|a, b| a.name.cmp(&b.name));
        // deserialized_function.parameters.sort_by(|a, b| a.name.cmp(&b.name));
        // assert_eq!(function, deserialized_function);        
    }
}