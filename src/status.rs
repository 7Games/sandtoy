pub enum StatusMessageType {
    Info = 0,
    Warning,
    Error,
}

pub struct StatusMessage {
    pub message_text: String,
    pub message_type: StatusMessageType,

    pub time_created: f64,
    pub destroy_timer: f64,
}

impl StatusMessage {
    pub fn new(
        message_text: String,
        message_type: StatusMessageType,
        time_created: f64,
        destroy_timer: f64,
    ) -> Self {
        Self {
            message_text,
            message_type,
            time_created,
            destroy_timer,
        }
    }
}

pub struct Status {
    messages: Vec<StatusMessage>,
}

impl Status {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    pub fn get_messages(&self) -> &Vec<StatusMessage> {
        &self.messages
    }

    pub fn add(
        &mut self,
        message_text: String,
        message_type: StatusMessageType,
        time_created: f64,
        destroy_timer: f64,
    ) {
        self.messages.push(StatusMessage::new(
            message_text,
            message_type,
            time_created,
            destroy_timer,
        ));
    }

    pub fn remove_at(&mut self, index: usize) {
        self.messages.remove(index);
    }
}
