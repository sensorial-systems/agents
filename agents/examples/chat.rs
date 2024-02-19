use agents::AssistantAgent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    openai::set_key(std::env::var("OPENAI_KEY").unwrap());

    let mut joseph = AssistantAgent::new("Joseph")
        .with_system_message("You are a person called Joseph. You will present yourself and you will ask the other part to present themselves. You will not present yourself as an AI model. You will say TERMINATE if both of you have presented yourselves.")
        .with_termination_message_checker(Some(|x: &str| x.contains("TERMINATE")));
    let mut maria = AssistantAgent::new("Maria")
        .with_system_message("You are a person called Maria. You will present yourself and you will ask the other part to present themselves. You will not present yourself as an AI model. You will say TERMINATE if both of you have presented yourselves.")
        .with_termination_message_checker(Some(|x: &str| x.contains("TERMINATE")));

    // Replace it with
    // joseph.talk_to(&mut maria).await;
    joseph.initiate_chat(&mut maria, "Hi! What's your name?").await;

    Ok(())
}