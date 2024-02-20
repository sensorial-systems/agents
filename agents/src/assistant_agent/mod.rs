use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};

use crate::Conversation;

pub struct AssistantAgent {
    name: String,
    system_message: String,
    conversation_callback: Option<Box<dyn Fn(&mut Conversation)>>,
    message_history: Vec<ChatCompletionMessage>
}

impl AssistantAgent {
    fn system_message(&self) -> ChatCompletionMessage {
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: self.system_message.clone(),
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

        Self { name, system_message, message_history, conversation_callback }
    }

    pub fn with_system_message(mut self, system_message: impl Into<String>) -> Self {
        self.system_message = system_message.into();
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
            .seed(0 as u64)
            .create()
            .await
            .unwrap();
        let message = chat_completion.choices.first().unwrap().message.clone();

        println!("{} (to {}):\n{}\n", self.name, recipient.name, message.content);

        self.send(recipient, message.content).await
    }

    #[async_recursion::async_recursion(?Send)]
    async fn send(&mut self, recipient: &mut Self, message: impl AsRef<str> + 'static) {
        let model = std::env::var("MODEL").unwrap();
        let mut sent_message = ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: message.as_ref().into(),
            name: None,
            function_call: None
        };
        self.message_history.push(sent_message.clone());

        let chat_completion = ChatCompletion::builder(&model, self.message_history_for_assistant(recipient))
            .temperature(0.0)
            .seed(0 as u64)
            .create()
            .await
            .unwrap();

        let returned_message = chat_completion.choices.first().unwrap().message.clone();
        println!("{} (to {}):\n{}\n", recipient.name, self.name, returned_message.content);
        self.message_history.push(returned_message.clone());

        sent_message.role = ChatCompletionMessageRole::Assistant;
        recipient.message_history.push(sent_message);
        if let Some(callback) = self.conversation_callback.as_ref() {
            let last_message = &self.message_history.last().as_ref().unwrap().content;
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
