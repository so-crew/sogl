use crate::model::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub content: Vec<Color>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas{width, height, content: vec![Color::default(); width*height]}
    }

    pub fn set_content(&mut self, x: usize, y: usize, c: &Color) {
        self.content[x * self.width + y] = c.clone();
    }
}