pub struct AssistantAgent {

}

impl AssistantAgent {
    pub fn new() -> Self {
        Self {}
    }

    pub fn initiate_chat(&self, _other: &Self, message: impl AsRef<str>) {
        let _message = message.as_ref();
    } 
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        todo!()
    }
}