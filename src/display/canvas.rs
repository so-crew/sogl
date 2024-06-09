use std::{usize, vec};

use crate::{error::Error, model::Color};

#[derive(Debug)]
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

    pub fn get_size(&self) -> usize {
        self.width * self.height
    }

    pub fn set_content(&mut self, coord: CanvasCoordinate, color: Color) -> Result<(), Error> {
        match coord {
            CanvasCoordinate::Cartesian(x, y) => {
                let idx = x * self.width + y;
                if idx >= self.width * self.height {
                    return Err(Error::OutOfBounds(coord));
                }
                self.content[idx] = color.clone();
                Ok(())
            }
            CanvasCoordinate::Linear(i) => {
                if i >= self.width * self.height {
                    return Err(Error::OutOfBounds(coord));
                }
                self.content[i] = color.clone();
                Ok(())
            }
        }
    }

    pub fn get_content(&self, coord: CanvasCoordinate) -> Result<Color, Error> {
        match coord {
            CanvasCoordinate::Cartesian(x, y) => {
                let idx = x * self.width + y;
                if idx >= self.width * self.height {
                    return Err(Error::OutOfBounds(coord));
                }
                Ok(self.content[idx].clone())
            }
            CanvasCoordinate::Linear(i) => {
                if i >= self.width * self.height {
                    return Err(Error::OutOfBounds(coord));
                }
                Ok(self.content[i].clone())
            }
        }
    }

    pub fn get_contents(&self) -> &Vec<Color> {
        &self.content
    }
}
