pub struct Conversation<'message> {
    message: &'message str,
    terminated: bool
}

impl<'message> Conversation<'message> {
    pub fn new(message: &'message str) -> Self {
        let message = message.into();
        let terminated = false;

        Self { message, terminated }
    }

    pub fn last_message(&self) -> &str {
        self.message
    }

    pub fn terminate(&mut self) {
        self.terminated = true;
    }

    pub fn is_terminated(&self) -> bool {
        self.terminated
    }
}