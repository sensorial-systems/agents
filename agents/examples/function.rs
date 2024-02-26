use agents::{AgentFunction, Agent, Conversation, FunctionParameter, Instruction};
use agents::models::{GPT4, OpenAIKeySrc};

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

fn quote_amount(base_amount: f32, base_currency: &str, quote_currency: &str) -> String {
    format!("{} {}", base_amount * exchange_rate(base_currency, quote_currency), quote_currency)
}

fn instruction_quote_amount(arguments: String) -> String {
    let arguments = serde_json::from_str::<serde_json::Value>(&arguments).unwrap();
    let base_amount = arguments.get("amount").unwrap().as_f64().unwrap() as f32;
    let base_currency = arguments.get("from").unwrap().as_str().unwrap();
    let quote_currency = arguments.get("to").unwrap().as_str().unwrap();
    quote_amount(base_amount, base_currency, quote_currency)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = GPT4::new(OpenAIKeySrc::DOTENV);

    let mut dealer = Agent::new(&model, "Currency Exchange Dealer")
            .with_instruction(
                Instruction::new("You are a currency exchange dealer.")
                    .with_functions(vec![
                        AgentFunction::new("quote_amount")
                            .with_callback(instruction_quote_amount)
                            .with_description("Quote the amount of money in a currency from another currency")
                            .with_parameters(vec![
                                FunctionParameter::number("amount").with_description("The amount of money to convert"),
                                FunctionParameter::string("from").with_description("The currency of origin"),
                                FunctionParameter::string("to").with_description("The currency of destination")
                            ])
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
