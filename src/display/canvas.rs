use crate::model::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub content: Vec<Color>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas{width, height, content: Vec::new()}
    }
}