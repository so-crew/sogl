pub use super::Canvas;

pub trait Displayer {
    fn show(self, c: &Canvas);
}