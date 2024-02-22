use crate::Message;

#[derive(Clone)]
pub struct Conversation {
    history: Vec<Message>,
    terminated: bool,
}

impl<'message> Conversation {
    pub fn new() -> Self {
        let terminated = false;
        let history = Default::default();
        Self { terminated, history }
    }

    pub fn history(&self) -> &Vec<Message> {
        &self.history
    }

    pub fn history_mut(&mut self) -> &mut Vec<Message> {
        &mut self.history
    }

    pub fn last_message(&self) -> String {
        self.history
            .last()
            .cloned()
            .map(|m| m.content)
            .unwrap_or_default()
    }

    pub fn last_message_mut(&mut self) -> Option<&mut String> {
        self
            .history
            .last_mut()
            .map(|m| &mut m.content)
    }

    pub fn terminate(&mut self) {
        self.terminated = true;
    }

    pub fn has_terminated(&self) -> bool {
        self.terminated
    }
}