use agents::{AgentFunction, Agent, Conversation, Instruction};
use agents::models::GPT4;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

fn exchange_rate(base_currency: &str, quote_currency: &str) -> f32 {
    if base_currency == quote_currency {
        1.0
    } else if base_currency == "USD" && quote_currency == "EUR" {
        1.1
    } else if base_currency == "EUR" && quote_currency == "USD" {
        1.0 / 1.1
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

fn instruction_quote_amount(arguments: String) -> String {
    let arguments = serde_json::from_str::<QuoteAmountParameters>(&arguments).unwrap();
    quote_amount(arguments)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("OPENAI_API_KEY").expect("Environment variable OPENAI_KEY is not set.");
    let model = GPT4::new(api_key);

    let mut dealer = Agent::new(&model, "Currency Exchange Dealer")
            .with_instruction(
                Instruction::new("You are a currency exchange dealer.")
                    .with_functions(vec![
                        AgentFunction::new("quote_amount")
                            .with_callback::<QuoteAmountParameters>(instruction_quote_amount)
                            .with_description("Quote the amount of money in a currency from another currency")
                    ])
            );

    let mut customer = Agent::new(&model, "Customer")
        .with_instruction("You are a customer. You will say \"Thank you\" if the question you asked is answered.")
        .with_notifications(Some(|conversation: &mut Conversation| {
            if conversation.last_message().content.as_text().map(|text| text.contains("Thank you")).unwrap_or(false) {
                conversation.terminate();
            }
        }));
    customer.initiate_chat(&mut dealer, "How much is 100 USD in EUR?").await;

    Ok(())
}
