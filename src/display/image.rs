use std::io::Write;

use crate::error::Error;

use super::{Canvas, Displayer, ERROR_COORDINATE_OUT_OF_RANGE};

pub const ERROR_IMAGE_FORMAT_NOT_SET: Error = Error {
    message: "image format not set",
};

const BITMAP_HEADER_LENGTH: usize = 14;
const BITMAP_INFO_HEADER_LENGTH: usize = 40;

pub enum BitmapBitsPerPixel {
    Monochrome = 0x0001,
    Palletize4Bit = 0x0004,
    Palletize8Bit = 0x0008,
    RGB16Bit = 0x0010,
    RGB24Bit = 0x0110,
}

pub enum BitmapCompression {
    BIRGB = 0x0000,
    BIRLE8 = 0x0001,
    BIRLE4 = 0x0002,
}

pub struct BitmapOptions {
    pub bits_per_pixel: BitmapBitsPerPixel,
    pub compression: BitmapCompression,
    pub x_pixels_per_meter: u32,
    pub y_pixels_per_meter: u32,
    pub colors_used: u32,
    pub important_colors: u32,
}

pub enum ImageFormat {
    Bitmap(BitmapOptions),
}

pub struct ImageDisplayBuilder<'a> {
    image_format: Option<ImageFormat>,
    output: Option<&'a mut dyn Write>,
}

pub struct ImageDisplay<'a> {
    image_format: ImageFormat,
    output: &'a mut dyn Write,
}

impl<'a> ImageDisplayBuilder<'a> {
    pub fn new() -> ImageDisplayBuilder<'a> {
        ImageDisplayBuilder {
            image_format: None,
            output: None,
        }
    }

    pub fn set_image_format(mut self, image_format: ImageFormat) -> ImageDisplayBuilder<'a> {
        self.image_format = Some(image_format);
        self
    }

    pub fn set_output(mut self, stream: &'a mut dyn Write) -> ImageDisplayBuilder<'a> {
        self.output = Some(stream);
        self
    }

    pub fn build(self) -> Result<ImageDisplay<'a>, Error> {
        Ok(ImageDisplay {
            image_format: match self.image_format {
                Some(image_format) => image_format,
                None => return Err(ERROR_COORDINATE_OUT_OF_RANGE),
            },
            output: match self.output {
                Some(stream) => stream,
                None => return Err(ERROR_IMAGE_FORMAT_NOT_SET),
            },
        })
    }
}

impl<'a> Displayer for ImageDisplay<'a> {
    fn show(self, c: &Canvas) {
        match self.image_format {
            ImageFormat::Bitmap(options) => {
                let pixels_size = c.get_height() * c.get_width() * options.bits_per_pixel as usize;
                let buffer_size = BITMAP_HEADER_LENGTH + BITMAP_HEADER_LENGTH + pixels_size;
                let buffer: Vec<u8> = (0..buffer_size).map(|i| -> u8 { todo!() }).collect();

                let _ = self.output.write(&buffer);
            }
        }
    }
}
