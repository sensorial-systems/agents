pub struct AssistantAgent {

}

impl AssistantAgent {
    pub fn new() -> Self {
        Self {}
    }

    pub fn initiate_chat(&self, _other: &Self, message: impl AsRef<str>) {
        let _message = message.as_ref();
    } 
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test() -> Result<(), Box<dyn std::error::Error>> {
        use openai::chat::*;

        dotenv::dotenv()?;
        openai::set_key(std::env::var("API_KEY").unwrap());

        let model = std::env::var("MODEL").unwrap();
        let messages = vec![
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: Some("You are a helpful assistant".into()),
                name: None,
                function_call: None
            },
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some("Hello!".into()),
                name: None,
                function_call: None
            }
        ];
        
        let chat_completion = ChatCompletion::builder(&model, messages)
            .create()
            .await
            .unwrap();
        let returned_message = chat_completion.choices.first().unwrap().message.clone();
        println!("{:#?}", returned_message);
        Ok(())
    }
}