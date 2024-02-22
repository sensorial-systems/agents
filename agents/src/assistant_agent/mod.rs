use openai::chat::{ChatCompletion, ChatCompletionFunctionDefinition, ChatCompletionMessage, ChatCompletionMessageRole};

use crate::{AgentFunction, Conversation};

pub struct AssistantAgent {
    name: String,
    system_message: String,
    conversation_callback: Option<Box<dyn Fn(&mut Conversation)>>,
    functions: Vec<AgentFunction>,
    message_history: Vec<ChatCompletionMessage>
}

impl AssistantAgent {
    fn system_message(&self) -> ChatCompletionMessage {
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some(self.system_message.clone()),
            name: None,
            function_call: None
        }
    }
}

impl AssistantAgent {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let system_message = Default::default();
        let message_history = Default::default();
        let conversation_callback = None;
        let functions = Default::default();

        Self { name, functions, system_message, message_history, conversation_callback }
    }

    pub fn with_system_message(mut self, system_message: impl Into<String>) -> Self {
        self.system_message = system_message.into();
        self
    }

    pub fn with_functions(mut self, functions: Vec<AgentFunction>) -> Self {
        self.functions = functions;
        self
    }

    pub fn with_conversation_callback(mut self, conversation_callback: Option<impl Fn(&mut Conversation) + 'static>) -> Self {
        self.conversation_callback = conversation_callback.map(|x| Box::new(x) as Box<dyn Fn(&mut Conversation)>);
        self
    }

    fn message_history_for_assistant(&self, assistant: &Self) -> Vec<ChatCompletionMessage> {
        vec![assistant.system_message()]
            .into_iter()
            .chain(self.message_history.clone())
            .collect()
    }

    pub async fn talk_to(&mut self, recipient: &mut Self) {
        let model = std::env::var("MODEL").unwrap();
        let chat_completion = ChatCompletion::builder(&model, self.message_history_for_assistant(self))
            .temperature(0.0)
            .create()
            .await
            .unwrap();
        let message = chat_completion.choices.first().unwrap().message.clone();

        println!("{} (to {}):\n{}\n", self.name, recipient.name, message.content.as_ref().unwrap());

        self.send(recipient, message.content.unwrap()).await
    }

    fn oai_functions(&self) -> Vec<openai::chat::ChatCompletionFunctionDefinition> {
        self.functions.iter().map(|f| {
            let function = serde_json::to_value(&f).unwrap();
            let parameters = function.get("parameters").unwrap().clone();
            ChatCompletionFunctionDefinition {
                name: f.name.clone(),
                description: Some(f.description.clone()),
                parameters: Some(parameters)
            }
        }).collect()
    }

    #[async_recursion::async_recursion(?Send)]
    async fn send(&mut self, recipient: &mut Self, message: impl AsRef<str> + 'static) {
        let model = std::env::var("MODEL").unwrap();
        let mut sent_message = ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(message.as_ref().into()),
            name: None,
            function_call: None
        };
        self.message_history.push(sent_message.clone());

        let chat_completion = ChatCompletion::builder(&model, self.message_history_for_assistant(recipient))
            .functions(recipient.oai_functions())
            .temperature(0.0)
            .create()
            .await
            .unwrap();

        let returned_message = chat_completion.choices.first().unwrap().message.clone();
        println!("{} (to {}):\n{:#?}\n", recipient.name, self.name, returned_message);
        self.message_history.push(returned_message.clone());

        sent_message.role = ChatCompletionMessageRole::Assistant;
        recipient.message_history.push(sent_message);
        if let Some(callback) = self.conversation_callback.as_ref() {
            let last_message = self.message_history.last().unwrap().content.as_ref().unwrap();
            let mut context = Conversation::new(&last_message);
            callback(&mut context);
            if !context.is_terminated() {
                recipient.send(self, last_message.clone()).await;
            }
        }
    }

    pub async fn initiate_chat(&mut self, recipient: &mut Self, message: impl AsRef<str> + 'static) {
        println!("{} (to {}):\n{}\n", self.name, recipient.name, message.as_ref());
        self.send(recipient, message).await;
    } 
}

#[cfg(test)]
mod tests {
    use openai::chat::{ChatCompletion, ChatCompletionFunctionDefinition, ChatCompletionMessage};

    #[tokio::test]
    async fn function_call() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv()?;
        openai::set_key(std::env::var("OPENAI_KEY").unwrap());
        let model = std::env::var("MODEL").unwrap();

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