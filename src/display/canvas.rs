use std::usize;

use crate::{error::Error, model::Color};

pub const ERROR_COORDINATE_OUT_OF_RANGE: Error = Error {
    message: "coordinate out of range",
};

pub enum CanvasCoordinate {
    Cartesian(usize, usize),
    Linear(usize),
}

pub struct Canvas {
    width: usize,
    height: usize,
    content: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            content: vec![Color::default(); width * height],
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn set_content(&mut self, coord: CanvasCoordinate, color: &Color) -> Result<(), Error> {
        match coord {
            CanvasCoordinate::Cartesian(x, y) => {
                let idx = x * self.width + y;
                if idx >= self.width * self.height {
                    return Err(ERROR_COORDINATE_OUT_OF_RANGE);
                }
                self.content[idx] = color.clone();
                Ok(())
            }
            CanvasCoordinate::Linear(i) => {
                if i >= self.width * self.height {
                    return Err(ERROR_COORDINATE_OUT_OF_RANGE);
                }
                self.content[i] = color.clone();
                Ok(())
            }
        }
    }

    pub fn get_content(&self, coord: CanvasCoordinate) -> Option<Color> {
        match coord {
            CanvasCoordinate::Cartesian(x, y) => {
                let idx = x * self.width + y;
                if idx >= self.width * self.height {
                    return None;
                }
                Some(self.content[idx].clone())
            }
            CanvasCoordinate::Linear(i) => {
                if i >= self.width * self.height {
                    return None;
                }
                Some(self.content[i].clone())
            }
        }
    }
}
