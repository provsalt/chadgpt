use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use openai::set_key;
use crate::Models;

pub struct API {
    model: Models,
    max_tokens: u32,
    messages: Vec<ChatCompletionMessage>,
}

impl API {
    pub fn new(key: String, model: Models, max_tokens: u32) -> API {
        set_key(key.trim().to_string());
        API {
            model,
            max_tokens,
            messages: Vec::new(),
        }
    }

    pub async fn send(&mut self, message: &str) -> String {
        self.messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: message.trim().to_string(),
            name: None,
        });
        let chat_completion = match self.model {
            Models::GPT35 => ChatCompletion::builder("gpt-3.5-turbo", self.messages.clone()),
            Models::GPT4 => ChatCompletion::builder("gpt-4", self.messages.clone()),
        }.max_tokens(self.max_tokens).create().await.unwrap().unwrap();
        let msg = chat_completion.choices.first().unwrap().message.clone();
        self.messages.push(msg);
        self.messages.last().unwrap().content.clone()
    }
}