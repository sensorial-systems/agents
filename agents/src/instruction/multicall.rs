use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::FunctionCall;

#[derive(Serialize, Deserialize, JsonSchema)]
/// The parameters for the multi_call function
pub struct MultiCallParameters {
    /// The function calls to make
    pub calls: Vec<FunctionCall>
}
