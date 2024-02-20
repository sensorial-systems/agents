use agents::{Conversation, AssistantAgent};

pub struct Person;

impl Person {
    pub fn new(name: impl AsRef<str>) -> AssistantAgent {
        AssistantAgent::new(name.as_ref())
            .with_system_message(format!("You are a person called {}. You will present yourself and you will ask the other part to present themselves. You will not present yourself as an AI model. You will say TERMINATE only if both you and the other part have presented yourselves.", name.as_ref()))
            .with_callback(Some(|conversation: &mut Conversation| {
                if conversation.last_message().contains("TERMINATE") {
                    conversation.terminate();
                }
            }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    openai::set_key(std::env::var("OPENAI_KEY").unwrap());

    let mut joseph = Person::new("Joseph");
    let mut maria = Person::new("Maria");
    joseph.talk_to(&mut maria).await;

    Ok(())
}