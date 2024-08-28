#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub context: Vec<String>,
}

impl ParseError {
    pub fn new(message: String) -> Self {
        ParseError {
            message,
            context: Vec::new(),
        }
    }

    pub fn add_context(&mut self, context: String) {
        self.context.push(context);
    }
}

