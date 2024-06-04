use std::io::Write;

use crate::error::Error;

use super::{Canvas, Displayer, ERROR_OUTPUT_NOT_SET};

pub const ERROR_IMAGE_FORMAT_NOT_SET: Error = Error {
    message: "image format not set",
};

const BITMAP_HEADER_LEN: usize = 14;
const BITMAP_INFO_HEADER_LEN: usize = 40;

const BITMAP_FILE_SIGNATURE: &[u8] = &[b'B', b'M'];
const BITMAP_NUMBER_OF_PLANE: &[u8] = &[0x01, 0x00];
const BITMAP_IMAGE_DATA_OFFET: &[u8] = &[0x36, 0x00, 0x00, 0x00];
const BITMAP_EMPTY_4BYTES: &[u8] = &[0x00, 0x00, 0x00, 0x00];

#[derive(Clone, Copy)]
pub enum BitmapBitsPerPixel {
    Monochrome,
    Palletize4Bit,
    Palletize8Bit,
    Rgb16Bit,
    Rgb24Bit,
}

impl BitmapBitsPerPixel {
    pub fn value(&self) -> u16 {
        match self {
            BitmapBitsPerPixel::Monochrome => 0x0001,
            BitmapBitsPerPixel::Palletize4Bit => 0x0004,
            BitmapBitsPerPixel::Palletize8Bit => 0x0008,
            BitmapBitsPerPixel::Rgb16Bit => 0x0010,
            BitmapBitsPerPixel::Rgb24Bit => 0x0018,
        }
    }

    pub fn byte_per_pixel(&self) -> usize {
        match self {
            BitmapBitsPerPixel::Monochrome => 0x0000,
            BitmapBitsPerPixel::Palletize4Bit => 0x0000,
            BitmapBitsPerPixel::Palletize8Bit => 0x0001,
            BitmapBitsPerPixel::Rgb16Bit => 0x0002,
            BitmapBitsPerPixel::Rgb24Bit => 0x0003,
        }
    }

    pub fn colors_used(&self) -> u32 {
        return 2_u32.pow(self.byte_per_pixel() as u32);
    }

    pub fn important_colors(&self) -> u32 {
        return 0;
    }
}

pub enum BitmapCompression {
    BIRGB,
    BIRLE8,
    BIRLE4,
}

impl BitmapCompression {
    fn value(&self) -> u32 {
        match *self {
            BitmapCompression::BIRGB => 0,
            BitmapCompression::BIRLE8 => 1,
            BitmapCompression::BIRLE4 => 2,
        }
    }
}

pub struct BitmapOptions {
    pub bits_per_pixel: BitmapBitsPerPixel,
    pub compression: BitmapCompression,
    pub x_pixels_per_meter: u32,
    pub y_pixels_per_meter: u32,
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
                None => return Err(ERROR_IMAGE_FORMAT_NOT_SET),
            },
            output: match self.output {
                Some(stream) => stream,
                None => return Err(ERROR_OUTPUT_NOT_SET),
            },
        })
    }
}

impl<'a> Displayer for ImageDisplay<'a> {
    fn show(&mut self, c: &Canvas) {
        match &self.image_format {
            ImageFormat::Bitmap(opt) => {
                let image_size = c.get_size() as u32 * opt.bits_per_pixel.byte_per_pixel() as u32;
                let file_size = (BITMAP_HEADER_LEN + BITMAP_INFO_HEADER_LEN) as u32 + image_size;

                // Write header
                let _ = self.output.write(
                    &[
                        &BITMAP_FILE_SIGNATURE[..],
                        &file_size.to_le_bytes(),
                        &BITMAP_EMPTY_4BYTES,
                        &BITMAP_IMAGE_DATA_OFFET,
                    ]
                    .concat(),
                );

                // Write info header
                let _ = self.output.write(
                    &[
                        &BITMAP_INFO_HEADER_LEN.to_le_bytes()[0..4],
                        &c.get_width().to_le_bytes()[0..4],
                        &c.get_height().to_le_bytes()[0..4],
                        &BITMAP_NUMBER_OF_PLANE[..],
                        &opt.bits_per_pixel.value().to_le_bytes(),
                        &opt.compression.value().to_le_bytes(),
                        &image_size.to_le_bytes(),
                        &opt.x_pixels_per_meter.to_le_bytes(),
                        &opt.y_pixels_per_meter.to_le_bytes(),
                        &opt.bits_per_pixel.colors_used().to_le_bytes(),
                        &opt.bits_per_pixel.important_colors().to_le_bytes(),
                    ]
                    .concat(),
                );

                // Write pixel data
                match opt.bits_per_pixel {
                    BitmapBitsPerPixel::Monochrome => todo!(),
                    BitmapBitsPerPixel::Palletize4Bit => todo!(),
                    BitmapBitsPerPixel::Palletize8Bit => {
                        let _ = self.output.write(
                            &c.get_contents()
                                .into_iter()
                                .map(|c| c.grayscale())
                                .collect::<Vec<u8>>(),
                        );
                    }
                    BitmapBitsPerPixel::Rgb16Bit => todo!(),
                    BitmapBitsPerPixel::Rgb24Bit => {
                        let _ = self.output.write(
                            &c.get_contents()
                                .into_iter()
                                .map(|c| [c.blue(), c.green(), c.red()])
                                .collect::<Vec<[u8; 3]>>()
                                .concat(),
                        );
                    }
                }
            }
        }
    }
}
