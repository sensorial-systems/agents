use agents::{AgentFunction, AssistantAgent, Conversation, FunctionParameter};

// fn exchange_rate(base_currency: &str, quote_currency: &str) -> f32 {
//     if base_currency == quote_currency {
//         1.0
//     } else if base_currency == "USD" && quote_currency == "EUR" {
//         1.1
//     } else if base_currency == "EUR" && quote_currency == "USD" {
//         1.0 / 1.1
//     } else {
//         0.0
//     }
// }

// fn quote_amount(base_amount: f32, base_currency: &str, quote_currency: &str) -> String {
//     format!("{} {}", base_amount * exchange_rate(base_currency, quote_currency), quote_currency)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    openai::set_key(std::env::var("OPENAI_KEY").unwrap());

    let mut dealer = AssistantAgent::new("Currency Exchange Dealer")
            .with_system_message("You are a currency exchange dealer.")
            .with_functions(vec![
                AgentFunction::new("quote_amount")
                    .with_description("Quote the amount of money in a currency from another currency")
                    .with_parameters(vec![
                        FunctionParameter::number("amount").with_description("The amount of money to convert"),
                        FunctionParameter::string("from").with_description("The currency of origin"),
                        FunctionParameter::string("to").with_description("The currency of destination")
                    ])
            ]);

    let mut customer = AssistantAgent::new("Customer")
        .with_system_message("You are a customer. You will say \"Thank you\" if the question you asked is answered.")
        .with_conversation_callback(Some(|conversation: &mut Conversation| {
            if conversation.last_message().contains("Thank you") {
                conversation.terminate();
            }
        }));
    customer.initiate_chat(&mut dealer, "How much is 100 USD in EUR?").await;

    Ok(())
}