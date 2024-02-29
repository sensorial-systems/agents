use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AgentFunction, FunctionCall};

#[derive(Serialize, Deserialize, JsonSchema)]
/// The parameters for the multi_call function
struct MultiCallParameters {
    /// The function calls to make
    pub calls: Vec<FunctionCall>
}

pub struct MultiCall;

impl From<MultiCall> for AgentFunction {
    fn from(_: MultiCall) -> Self {
        AgentFunction::new("multicall", move |registry: &crate::FunctionsRegistry, parameters: MultiCallParameters| {
            let mut output = Vec::new();
            for call in parameters.calls {
                if let Some(result) = registry.call(&call) {
                    output.push(result);
                }
            }
            output.join(", ")
        }).with_description("Call multiple functions at once.")
    }
}
