use std::io::Write;
use std::u8;
use std::vec;

use crate::error::Error;

use super::error::CHARSET_NOT_SET_ERROR;
use super::error::OUTPUT_NOT_SET_ERROR;
use super::Canvas;
use super::Displayer;

pub const DEFAULT_CHARSET: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

pub struct TextDisplay<'a> {
    charset: &'static str,
    output_stream: &'a mut dyn Write,
}

pub struct TextDisplayBuilder<'a> {
    charset: &'static str,
    output_stream: Option<&'a mut dyn Write>,
}

impl<'a> TextDisplayBuilder<'a> {
    pub fn new() -> TextDisplayBuilder<'a> {
        TextDisplayBuilder{
            charset: "",
            output_stream: None,
        }
    }

    pub fn set_charset(mut self, charset: &'static str) -> TextDisplayBuilder<'a> {
        self.charset = charset;
        self
    }

    pub fn set_output_stream(mut self, stream: &'a mut dyn Write) -> TextDisplayBuilder<'a> {
        self.output_stream = Some(stream);
        self
    }

    pub fn build(self) -> Result<TextDisplay<'a>, Error> {
        if self.charset.len() == 0 {
            return Err(CHARSET_NOT_SET_ERROR);
        }
        
        match self.output_stream {
            Some(stream) => Ok(TextDisplay{
                charset: self.charset,
                output_stream: stream,
            }),
            None => Err(OUTPUT_NOT_SET_ERROR),
        }
    }
}

impl<'a> Displayer for TextDisplay<'a> {
    fn show(self, c: &Canvas) {
        let charset_len = self.charset.len();
        let buffer_size = c.height * (c.width + 1);
        let mut buffer = vec![u8::default(); buffer_size];

        for i in 0..buffer_size {
            if i % (c.width + 1) == 0 {
                buffer[i] = b'\n';
                continue;
            }

            match c.content.get(i - i / (c.width + 1)) {
                Some(color) => {
                    let value_index = color.grayscale() as usize * charset_len as usize / u8::MAX as usize;
                    let value = self.charset.chars().nth(value_index).unwrap();
                    buffer[i] = value as u8;
                },
                None => buffer[i] = b' ',
            }
        }
        
        let _ = self.output_stream.write(&buffer);
    }
}