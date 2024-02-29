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

    pub fn add_message(&mut self, message: Message) {
        println!("{}", message);
        self.history.push(message);
    }

    pub fn history(&self) -> &Vec<Message> {
        &self.history
    }

    pub fn history_mut(&mut self) -> &mut Vec<Message> {
        &mut self.history
    }

    pub fn last_message(&self) -> Option<&Message> {
        self.history
            .last()
    }

    pub fn terminate(&mut self) {
        self.terminated = true;
    }

    pub fn has_terminated(&self) -> bool {
        self.terminated
    }
}