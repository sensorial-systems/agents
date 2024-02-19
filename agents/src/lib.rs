use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};

pub struct AssistantAgent {
    name: String,
    system_message: String,
    termination_message_checker: Option<Box<dyn Fn(&str) -> bool>>,
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
        let termination_message_checker = None;
        let message_history = Default::default();

        Self { name, system_message, termination_message_checker, message_history }
    }

    pub fn with_system_message(mut self, system_message: impl Into<String>) -> Self {
        self.system_message = system_message.into();
        self
    }

    pub fn with_termination_message_checker(mut self, checker: Option<impl Fn(&str) -> bool + 'static>) -> Self {
        self.termination_message_checker = checker.map(|x| Box::new(x) as Box<dyn Fn(&str) -> bool>);
        self
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

        let mut messages = vec![recipient.system_message()];
        messages.extend(self.message_history.iter().cloned());
        let chat_completion = ChatCompletion::builder(&model, messages.clone())
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
        if let Some(checker) = self.termination_message_checker.as_ref() {
            let last_message = &self.message_history.last().as_ref().unwrap().content;
            if !checker(&last_message) {
                recipient.send(self, last_message.clone()).await;
            }
        }
    }

    pub async fn initiate_chat(&mut self, recipient: &mut Self, message: impl AsRef<str> + 'static) {
        println!("{} (to {}):\n{}\n", self.name, recipient.name, message.as_ref());
        self.send(recipient, message).await;
    } 
}
