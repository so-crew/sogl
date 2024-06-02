use crate::error::Error;

pub use super::Canvas;

pub const ERROR_OUTPUT_NOT_SET: Error = Error {
    message: "output not set",
};

pub trait Displayer {
    fn show(&mut self, c: &Canvas);
}
