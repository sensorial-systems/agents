use openai::chat::ChatCompletion;

use crate::{Conversation, Instruction, Message};

// The lifetimes in this module tie the OpenAI API key to the GPT4 instance's scope.
#[derive(Clone)]
pub enum OpenAIKeySrc <'a> {
    DOTENV,
    SYSENV,
    USERIN,
    CUSTOM(&'a str),
}

#[derive(Clone)]
pub struct GPT4 <'a> {
    pub api_key: OpenAIKeySrc <'a>
}

impl GPT4 <'_>{
    pub fn new() -> Self {
        Self { api_key: OpenAIKeySrc::USERIN }
    }

    pub fn key_src(&mut self, key_src: OpenAIKeySrc) -> &Self {
        self.api_key = key_src;
        self
    }

    fn get_key(&self) -> String {
        match self.api_key {
            OpenAIKeySrc::DOTENV => {
                handle_env_var_err(
                    dotenv::var("OPENAI_API_KEY")
                )
            },
            OpenAIKeySrc::SYSENV => {
                handle_env_var_err(
                    std::env::var("OPENAI_API_KEY")
                )
            },
            OpenAIKeySrc::USERIN => {
                prompt_user_for_key()
            },
            OpenAIKeySrc::CUSTOM(s) => String::from(s),
        }
    }

    pub fn name(&self) -> &str {
        "gpt-4"
    }

    pub async fn complete(&self, instruction: &Instruction, conversation: &Conversation) -> Message {
        openai::set_key(self.get_key());
        let messages = std::iter::once(instruction.message())
            .chain(conversation.history().iter().cloned().map(|x| x.into()))
            .collect::<Vec<_>>();
        let chat_completion = ChatCompletion::builder(self.name(), messages)
            .functions(instruction.functions())
            .temperature(0.0)
            .create()
            .await
            .unwrap();
        let message = chat_completion.choices.first().unwrap().message.clone();
        message.into()
    }
}

impl AsRef<GPT4<'_>> for GPT4 <'_> {
    fn as_ref(&self) -> &GPT4 {
        self
    }
}

fn prompt_user_for_key() -> String {
    todo!("prompt user and return String");
    String::new()   
}

fn handle_env_var_err<E: std::error::Error>(r: Result<String, E>) -> String {
    r.unwrap_or_else(|e| {
        eprintln!("\x1b[0;1;31mError:\x1b[0m {}", e);
        prompt_user_for_key()
    })
}
