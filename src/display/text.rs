use std::io::Write;
use std::u8;

use super::Canvas;
use super::Displayer;

pub const DEFAULT_CHARSET: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

pub struct TextDisplay<'a> {
    charset: &'static str,
    output_stream: Box<&'a mut dyn Write>,
}

pub struct TextDisplayBuilder<'a> {
    charset: &'static str,
    output_stream: Option<Box<&'a mut dyn Write>>,
}

impl<'a> TextDisplayBuilder<'a> {
    pub fn new() -> TextDisplayBuilder<'a> {
        TextDisplayBuilder{
            charset: "",
            output_stream: None,
        }
    }

    pub fn default() -> TextDisplayBuilder<'a> {
        TextDisplayBuilder{
            charset: DEFAULT_CHARSET,
            output_stream: None,
        }
    }

    pub fn set_output_stream(&self, stream: Box<&'a mut dyn Write>) -> TextDisplayBuilder<'a> {
        TextDisplayBuilder {
            charset: self.charset,
            output_stream: Some(stream),
        }
    }

    pub fn build(self) -> TextDisplay<'a> {
        match self.output_stream {
            Some(stream) => TextDisplay {
                charset: self.charset,
                output_stream: stream,
            },
            None => todo!(),
        }
    }
}

impl<'a> Displayer for TextDisplay<'a> {
    fn show(mut self, c: &Canvas) {
        let charset_len = self.charset.len();

        let mut buffer: String = String::new();
        for i in 0..c.height {
            for j in 0..c.width {
                let value = c.content.get(i * c.height + j);
                match value {
                    Some(color) => {
                        let idx = color.grayscale() as usize * charset_len as usize / u8::MAX as usize;
                        let pixel = self.charset.to_string().chars().nth(idx).unwrap();
                        buffer += pixel.to_string().as_str();
                    },
                    None => buffer += " ",
                }
            }
            buffer += "\n";
        }
        
        let _ = self.output_stream.write(buffer.as_bytes());
    }
}