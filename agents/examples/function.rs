
use agents::models::{GPT4, OpenAIKeySrc};
use agents::{Agent, AgentFunction, Conversation, FunctionCall, Instruction};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

fn exchange_rate(base_currency: &str, quote_currency: &str) -> f32 {
    if base_currency == quote_currency {
        1.0
    } else if base_currency == "USD" && quote_currency == "EUR" {
        1.1
    } else if base_currency == "USD" && quote_currency == "JPY" {
        150.0
    } else {
        0.0
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
/// The parameters for the quote_amount function
struct QuoteAmountParameters {
    /// The amount of money to convert
    amount: f32,
    /// The currency of origin
    from: String,
    /// The currency of destination
    to: String
}

fn quote_amount(parameters: QuoteAmountParameters) -> String {
    format!("{} {}", parameters.amount * exchange_rate(&parameters.from, &parameters.to), parameters.to)
}

#[derive(Serialize, Deserialize, JsonSchema)]
/// The parameters for the multi_call function
struct MultiCallParameters {
    /// The function calls to make
    calls: Vec<FunctionCall>
}

fn multicall(parameters: MultiCallParameters) -> String {
    parameters.calls.iter().map(|call| {
        let quote_amount_paramters = serde_json::from_value(call.arguments.clone()).unwrap();
        quote_amount(quote_amount_paramters)
    }).collect::<Vec<String>>().join(", ")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = GPT4::new();
    let mut dealer = Agent::new(&model, "Currency Exchange Dealer")
            .with_instruction(
                Instruction::new("You are a currency exchange dealer.")
                    .with_functions(vec![
                        AgentFunction::new("quote_amount", quote_amount)
                            .with_description("Quote the amount of money in a currency from another currency"),
                        AgentFunction::new("multicall", multicall)
                            .with_description("Call multiple functions at once")
                    ])
            );

    let mut customer = Agent::new(&model, "Customer")
        .with_instruction("You are a customer. You will say \"Thank you\" if the question you asked is answered.")
        .with_notifications(Some(|conversation: &mut Conversation| {
            if conversation.last_message().content.as_text().map(|text| text.contains("Thank you")).unwrap_or(false) {
                conversation.terminate();
            }
        }));
    customer.initiate_chat(&mut dealer, "How much is 100 USD in EUR? And in JPY?").await;

    Ok(())
}
