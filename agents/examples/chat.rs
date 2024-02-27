use agents::{models::GPT4, Agent, Conversation};

pub struct Person;

impl Person {
    pub fn new(model: &GPT4, name: impl AsRef<str>) -> Agent {
        Agent::new(model.clone(), name.as_ref())
            .with_instruction(format!("You are a person called {}. You will present yourself and you will ask the other part to present themselves. You will not present yourself as an AI model. You will say TERMINATE only if both you and the other part have presented yourselves.", name.as_ref()))
            .with_notifications(Some(|conversation: &mut Conversation| {
                if conversation.last_message().content.as_text().map(|text| text.contains("TERMINATE")).unwrap_or_default() {
                    conversation.terminate();
                }
            }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = GPT4::new(dotenv::var("OPENAI_API_KEY").unwrap());

    let mut joseph = Person::new(&model, "Joseph");
    let mut maria = Person::new(&model, "Maria");
    joseph.talk_to(&mut maria).await;

    Ok(())
}
