pub trait Task {
    fn execute(&self);
}

pub struct EmailTask {
    pub recipient: String,
}

pub struct LogTask {
    pub message: String,
}

impl Task for EmailTask {
    fn execute(&self) {
        println!("Sending email to: {}", self.recipient);
    }
}

impl Task for LogTask {
    fn execute(&self) {
        println!("Writing to log file: {}", self.message);
    }
}
