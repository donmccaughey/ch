use std::error;
use std::fmt;


#[derive(Debug)]
pub struct LibCError;

impl LibCError {
    pub fn new() -> LibCError {
        LibCError {}
    }

    pub fn from_errno() -> LibCError {
        LibCError::new()
    }
}


impl fmt::Display for LibCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR")
    }
}


impl error::Error for LibCError {
    fn description(&self) -> &str {
        "ERROR"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
