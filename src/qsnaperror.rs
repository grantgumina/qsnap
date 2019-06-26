use std::fmt;
use std::error::Error;
use std::result;

#[derive(Debug)]
pub struct QsnapError {
    message: String,
}

impl QsnapError {
    pub fn new(msg: &str) -> QsnapError {
        QsnapError{message: msg.to_string()}
    }
}

impl fmt::Display for QsnapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "{}", 
            self.message
        )
    }
}

impl Error for QsnapError {
    fn description(&self) -> &str {
        &self.message
    }
}

