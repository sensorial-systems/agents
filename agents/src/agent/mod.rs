use crate::{models::GPT4, Conversation, Instruction, Message};

pub struct Agent {
    name: String,
    model: GPT4,
    instruction: Instruction,
    notifications: Option<Box<dyn Fn(&mut Conversation)>>
}

impl Agent {
    pub fn new(model: impl AsRef<GPT4>, name: impl Into<String>) -> Self {
        let model = model.as_ref().clone();
        let name = name.into();
        let instruction = Default::default();
        let notifications = None;

        Self { model, name, instruction, notifications }
    }

    pub fn with_notifications(mut self, notifications: Option<impl Fn(&mut Conversation) + 'static>) -> Self {
        self.notifications = notifications.map(|x| Box::new(x) as Box<dyn Fn(&mut Conversation)>);
        self
    }

    pub fn with_instruction(mut self, instruction: impl Into<Instruction>) -> Self {
        self.instruction = instruction.into();
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn talk_to(&mut self, recipient: &mut Self) {
        let mut conversation = Conversation::new();
        self.talk_to_in(recipient, &mut conversation).await;
    }

    async fn talk_to_in(&mut self, recipient: &mut Self, conversation: &mut Conversation) {
        let mut message = self.model.complete(&self.instruction, conversation).await;
        if message.content.is_function_call() {
            message.sign(self, self);
        } else {
            message.sign(self, recipient);
        }
        conversation.add_message(message);

        if let Some(notifications) = &self.notifications {
            notifications(conversation);
        }
        if let Some(function_call) = conversation.last_message().content.as_function_call() {
            if let Some(result) = self.instruction.functions.call(function_call) {
                let mut message = Message::from(result);
                message.sign(self, self); // From should be an executor agent. It, for example, could be a non-LLM agent.
                conversation.add_message(message);
                recipient.pass_turn_to(self, conversation).await;
            }
        } else if !conversation.has_terminated() {
            self.pass_turn_to(recipient, conversation).await
        }
    }

    #[async_recursion::async_recursion(?Send)]
    async fn pass_turn_to(&mut self, recipient: &mut Self, conversation: &mut Conversation) {
        recipient.talk_to_in(self, conversation).await;
    }

    pub async fn initiate_chat<'a>(&'a mut self, recipient: &mut Self, message: impl Into<Message>) {
        let mut conversation = Conversation::new();
        let mut message = message.into();
        message.sign(self, recipient);
        conversation.add_message(message);
        self.pass_turn_to(recipient, &mut conversation).await;
    }
}

#[cfg(test)]
mod tests {
    use openai::chat::{ChatCompletion, ChatCompletionFunctionDefinition, ChatCompletionMessage};

    #[tokio::test]
    async fn function_call() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv()?;
        openai::set_key(std::env::var("OPENAI_KEY").unwrap());
        let model = "gpt-4";

        let messages = vec![
            ChatCompletionMessage {
                role: openai::chat::ChatCompletionMessageRole::User,
                content: Some("Convert 5 USD to EUR and 3 BRL to 7 JPY".to_string()),
                name: None,
                function_call: None
            }
        ];

        let chat_completion = ChatCompletion::builder(&model, messages)
            .temperature(0.0)
            .functions([ChatCompletionFunctionDefinition {
                name: "convert_currency".to_string(),
                description: Some("Convert currencies".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "amount": {
                            "type": "number",
                            "description": "The amount of money to convert"
                        },
                        "origin": {
                            "type": "string",
                            "description": "The currency of origin"
                        },
                        "destination": {
                            "type": "string",
                            "description": "The currency of destination"
                        },
                    }
                })),
            }])
            .create()
            .await
            .unwrap();

        for choice in chat_completion.choices {
            let returned_message = choice.message.clone();
            let function_call = returned_message.function_call.unwrap();
            println!("{}({})", function_call.name, function_call.arguments);
        }

        Ok(())
    }
}