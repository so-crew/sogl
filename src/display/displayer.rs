use crate::error::Result;

pub use super::Canvas;

pub trait Displayer {
    fn show(&mut self, c: &Canvas) -> Result<usize>;
}
