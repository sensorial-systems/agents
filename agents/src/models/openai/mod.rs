use openai::chat::ChatCompletion;
pub mod api_key;
use api_key::*;

use crate::{Conversation, Instruction, Message};

#[derive(Clone)]
pub struct GPT4 {
    api_src: OpenAIKeySrc,
    cached_src: Option<OpenAIKeySrc>,
    cached_key: String,
    fallback: Option<OpenAIKeySrc>,
    human_fallback_enabled: bool,
}

impl GPT4 {
    pub fn new() -> Self {
        // default sets no fallback and gets key from .env file
        Self { 
            api_src: OpenAIKeySrc::DOTENV,
            cached_src: None,
            cached_key: String::new(),
            fallback: None,
            human_fallback_enabled: false,
        }
    }

    pub fn with_key(&mut self, key_src: OpenAIKeySrc) -> &Self {
        self.api_src = key_src;
        self
    }

    pub fn fallback_key(&mut self, fallback: Option<OpenAIKeySrc>) -> &Self {
        self.fallback = fallback;
        self
    } 

    pub fn human_fallback(&mut self, human_fallback: bool) -> &Self {
        self.human_fallback_enabled = human_fallback;
        self
    }

    fn get_key(&mut self) -> String {
        // These 11 lines create a duplicate of the api_src flag that only gets the API key when
        // it changes.
        if let Some(src) = self.cached_src.clone() {    // if `cached_src` is set &&
            if src == self.api_src {                    // if `api_key` isn't different from its last value
                return self.cached_key.clone();         // return the cached value
            }
        }
        self.cached_src = Some(self.api_src.clone());   // copy the changed src location to the `cached_src`

        // then cache the key into `cached_key` and return it
        self.cached_key = match self.api_src.clone() {
            OpenAIKeySrc::DOTENV => {
                handle_fallback(
                    dotenv::var("OPENAI_API_KEY"),
                    self.fallback.clone(),
                    self.human_fallback_enabled,
                )
            },
            OpenAIKeySrc::SYSENV => {
                handle_fallback(
                    std::env::var("OPENAI_API_KEY"), 
                    self.fallback.clone(),
                    self.human_fallback_enabled,
                )
            },
            OpenAIKeySrc::USERIN => {
                handle_fallback(
                    prompt_user_for_key(),
                    self.fallback.clone(),
                    self.human_fallback_enabled,
                )
            },
            OpenAIKeySrc::CUSTOM(s) => s,
        };
        self.cached_key.clone()
    }

    pub fn name(&self) -> &str {
        "gpt-4"
    }

    pub async fn complete(&mut self, instruction: &Instruction, conversation: &Conversation) -> Message {
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

impl AsRef<GPT4> for GPT4 {
    fn as_ref(&self) -> &GPT4 {
        self
    }
}
