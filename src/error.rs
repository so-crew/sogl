#[derive(Clone, Copy, Debug)]
pub struct Error {
    pub message: &'static str
}

impl Error {
    pub fn new(message: &'static str) -> Error {
        Error{message}
    }
}