use agents::{AgentFunction, Communicator, Conversation, AutoAgent, FunctionsRegistry, Instruction, MultiCall};
use agents::models::GPT4;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

fn exchange_rate(base_currency: &str, quote_currency: &str) -> f32 {
    if base_currency == quote_currency {
        1.0
    } else if base_currency == "BRL" && quote_currency == "EUR" {
        0.2
    } else if base_currency == "BRL" && quote_currency == "JPY" {
        30.0
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

fn quote_amount(_registry: &FunctionsRegistry, parameters: QuoteAmountParameters) -> String {
    format!("{} {}", parameters.amount * exchange_rate(&parameters.from, &parameters.to), parameters.to)
}

#[tokio::test]
async fn function() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("OPENAI_API_KEY").expect("Environment variable OPENAI_KEY is not set.");
    let model = GPT4::new(api_key);
    let mut dealer = AutoAgent::new(&model, "Currency Exchange Dealer")
        .with_instruction(
            Instruction::new("You are a currency exchange dealer.")
                .with_functions(vec![
                    AgentFunction::new("quote_amount", quote_amount)
                        .with_description("Quote the amount of money in a currency from another currency"),
                    MultiCall.into()
                ])
        )
        .with_notifications(Some(|conversation: &mut Conversation| {
            if let Some(last_message) = conversation.last_message() {
                if last_message.content.as_text().map(|text| text.contains("Thank you")).unwrap_or(false) {
                    conversation.terminate();
                }
            } 
        }));
    

    let mut customer = AutoAgent::new(&model, "Customer")
        .with_instruction("You are a customer. You will say \"Thank you\" if the question you asked is answered.");
    let mut conversation = Conversation::new();
    customer.send(&mut dealer, &mut conversation, "How much is 100 BRL in EUR? And in JPY?".into()).await;
    Ok(())
}
