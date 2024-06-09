use std::io::stdout;
use std::io::Stdout;
use std::io::Write;
use std::u8;

use crate::error::Error;
use crate::error::Result;
use crate::model::Color;

use super::Canvas;
use super::CanvasCoordinate;
use super::Displayer;

pub const DEFAULT_CHARSET: &str =
    " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

#[derive(Debug)]
pub struct TextDisplay<T: Write> {
    charset: &'static str,
    charset_len: usize,
    output: T,
}

pub struct TextDisplayBuilder<T: Write = Stdout> {
    charset: &'static str,
    output: Option<T>,
}

impl Default for TextDisplayBuilder {
    fn default() -> Self {
        Self {
            charset: DEFAULT_CHARSET,
            output: Some(stdout()),
        }
    }
}

impl TextDisplayBuilder {
    pub fn new() -> TextDisplayBuilder {
        TextDisplayBuilder {
            charset: "",
            output: None,
        }
    }

    pub fn set_charset(mut self, charset: &'static str) -> TextDisplayBuilder {
        self.charset = charset;
        self
    }

    pub fn set_output<T: Write>(self, stream: T) -> TextDisplayBuilder<T> {
        TextDisplayBuilder {
            charset: self.charset,
            output: Some(stream),
        }
    }
}

impl<T: Write> TextDisplayBuilder<T> {
    pub fn build(self) -> Result<TextDisplay<T>> {
        if self.charset.is_empty() {
            return Err(Error::MissingParams("charset not set".to_owned()));
        }

        match self.output {
            Some(stream) => Ok(TextDisplay {
                charset: self.charset,
                charset_len: self.charset.len(),
                output: stream,
            }),
            None => Err(Error::MissingParams("output not set".to_owned())),
        }
    }
}

impl<T: Write> TextDisplay<T> {
    pub fn color_to_char(&self, color: &Color) -> char {
        let value_index = color.intensity() as usize * self.charset_len / u8::MAX as usize;
        self.charset
            .chars()
            .nth(value_index)
            .unwrap_or(self.charset.chars().last().unwrap_or(' '))
    }
}

impl<T: Write> Displayer for TextDisplay<T> {
    fn show(&mut self, c: &Canvas) -> Result<usize> {
        let line_width = c.get_width() + 1;
        let buffer_size = c.get_height() * line_width;
        let buffer: Vec<u8> = (0..buffer_size)
            .map(|i| -> u8 {
                if i % line_width == 0 {
                    return b'\n';
                }
                match c.get_content(CanvasCoordinate::Linear(i - i / line_width)) {
                    Ok(color) => self.color_to_char(&color) as u8,
                    Err(_) => b' ',
                }
            })
            .collect();

        self.output.write(&buffer).map_err(Error::from)
    }
}
