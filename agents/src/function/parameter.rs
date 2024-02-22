use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    Number,
    String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FunctionParameter {
    #[serde(skip)]
    pub name: String,
    pub type_: ParameterType,
    pub description: String,
}

impl FunctionParameter {
    pub fn number(name: impl Into<String>) -> Self {
        let name = name.into();
        let type_ = ParameterType::Number;
        let description = Default::default();
        Self { name, type_, description }
    }

    pub fn string(name: impl Into<String>) -> Self {
        let name = name.into();
        let type_ = ParameterType::String;
        let description = Default::default();
        Self { name, type_, description }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

}
