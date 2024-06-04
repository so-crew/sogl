use std::io::Write;
use std::u8;

use crate::error::Error;
use crate::model::Color;

use super::Canvas;
use super::CanvasCoordinate;
use super::Displayer;
use super::ERROR_OUTPUT_NOT_SET;

pub const ERROR_CHARSET_NOT_SET: Error = Error {
    message: "charset not set",
};

pub const DEFAULT_CHARSET: &str =
    " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

pub struct TextDisplay<'a> {
    charset: &'static str,
    charset_len: usize,
    output: &'a mut dyn Write,
}

pub struct TextDisplayBuilder<'a> {
    charset: &'static str,
    output: Option<&'a mut dyn Write>,
}

impl<'a> TextDisplayBuilder<'a> {
    pub fn new() -> TextDisplayBuilder<'a> {
        TextDisplayBuilder {
            charset: "",
            output: None,
        }
    }

    pub fn set_charset(mut self, charset: &'static str) -> TextDisplayBuilder<'a> {
        self.charset = charset;
        self
    }

    pub fn set_output(mut self, stream: &'a mut dyn Write) -> TextDisplayBuilder<'a> {
        self.output = Some(stream);
        self
    }

    pub fn build(self) -> Result<TextDisplay<'a>, Error> {
        if self.charset.len() == 0 {
            return Err(ERROR_CHARSET_NOT_SET);
        }

        match self.output {
            Some(stream) => Ok(TextDisplay {
                charset: self.charset,
                charset_len: self.charset.len(),
                output: stream,
            }),
            None => Err(ERROR_OUTPUT_NOT_SET),
        }
    }
}

impl<'a> TextDisplay<'a> {
    pub fn color_to_char(&self, color: &Color) -> char {
        let value_index = color.grayscale() as usize * self.charset_len / u8::MAX as usize;
        self.charset
            .chars()
            .nth(value_index)
            .unwrap_or(self.charset.chars().last().unwrap_or(' '))
    }
}

impl<'a> Displayer for TextDisplay<'a> {
    fn show(&mut self, c: &Canvas) {
        let line_width = c.get_width() + 1;
        let buffer_size = c.get_height() * line_width;
        let buffer: Vec<u8> = (0..buffer_size)
            .map(|i| -> u8 {
                if i % line_width == 0 {
                    return b'\n';
                }

                match c.get_content(CanvasCoordinate::Linear(i - i / line_width)) {
                    Some(color) => self.color_to_char(&color) as u8,
                    None => b' ',
                }
            })
            .collect();

        let _ = self.output.write(&buffer);
    }
}
