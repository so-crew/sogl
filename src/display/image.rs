use std::{fs::File, io::Write};

use crate::error::Error;

use super::{Canvas, Displayer, ERROR_OUTPUT_NOT_SET};

pub const ERROR_IMAGE_FORMAT_NOT_SET: Error = Error {
    message: "image format not set",
};

pub const DEFAULT_IMAGE_FORMAT: ImageFormat = ImageFormat::Bitmap(BitmapOptions {
    bits_per_pixel: BitmapBitsPerPixel::Palletize4Bit,
    compression: BitmapCompression::BIRGB,
    x_pixels_per_meter: u32::MAX,
    y_pixels_per_meter: u32::MAX,
});

const BITMAP_HEADER_LEN: u32 = 14;
const BITMAP_INFO_HEADER_LEN: u32 = 40;

const BITMAP_FILE_SIGNATURE: u16 = 0x4D42;
const BITMAP_NUMBER_OF_PLANE: u16 = 0x0001;
const BITMAP_IMAGE_DATA_OFFET: u32 = 0x00000036;
const BITMAP_EMPTY_4BYTES: u32 = 0x00000000;

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

pub struct ImageDisplayBuilder<T: Write = File> {
    image_format: Option<ImageFormat>,
    output: Option<T>,
}

pub struct ImageDisplay<T: Write = File> {
    image_format: ImageFormat,
    output: T,
}

impl Default for ImageDisplayBuilder {
    fn default() -> ImageDisplayBuilder {
        ImageDisplayBuilder {
            image_format: Some(DEFAULT_IMAGE_FORMAT),
            output: match File::create("out.bmp") {
                Ok(stream) => Some(stream),
                Err(_) => None,
            },
        }
    }
}

impl ImageDisplayBuilder {
    pub fn new() -> ImageDisplayBuilder {
        ImageDisplayBuilder {
            image_format: None,
            output: None,
        }
    }

    pub fn set_image_format(mut self, image_format: ImageFormat) -> ImageDisplayBuilder {
        self.image_format = Some(image_format);
        self
    }

    pub fn set_output<T: Write>(self, stream: T) -> ImageDisplayBuilder<T> {
        ImageDisplayBuilder {
            image_format: self.image_format,
            output: Some(stream),
        }
    }
}

impl<T: Write> ImageDisplayBuilder<T> {
    pub fn build(self) -> Result<ImageDisplay<T>, Error> {
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

impl<T: Write> Displayer for ImageDisplay<T> {
    fn show(&mut self, c: &Canvas) {
        match &self.image_format {
            ImageFormat::Bitmap(opt) => {
                let image_size = c.get_size() as u32 * opt.bits_per_pixel.byte_per_pixel() as u32;
                let file_size = BITMAP_HEADER_LEN + BITMAP_INFO_HEADER_LEN + image_size;

                // Write header
                let _ = self.output.write(
                    [
                        BITMAP_FILE_SIGNATURE.to_le_bytes().as_slice(),
                        file_size.to_le_bytes().as_slice(),
                        BITMAP_EMPTY_4BYTES.to_le_bytes().as_slice(),
                        BITMAP_IMAGE_DATA_OFFET.to_le_bytes().as_slice(),
                    ]
                    .concat()
                    .as_slice(),
                );

                // Write info header
                let _ = self.output.write(
                    [
                        BITMAP_INFO_HEADER_LEN.to_le_bytes().as_slice(),
                        c.get_width().to_le_bytes().split_at(4).0,
                        c.get_height().to_le_bytes().split_at(4).0,
                        BITMAP_NUMBER_OF_PLANE.to_le_bytes().as_slice(),
                        opt.bits_per_pixel.value().to_le_bytes().as_slice(),
                        opt.compression.value().to_le_bytes().as_slice(),
                        image_size.to_le_bytes().as_slice(),
                        opt.x_pixels_per_meter.to_le_bytes().as_slice(),
                        opt.y_pixels_per_meter.to_le_bytes().as_slice(),
                        opt.bits_per_pixel.colors_used().to_le_bytes().as_slice(),
                        opt.bits_per_pixel
                            .important_colors()
                            .to_le_bytes()
                            .as_slice(),
                    ]
                    .concat()
                    .as_slice(),
                );

                // Write pixel data
                match opt.bits_per_pixel {
                    BitmapBitsPerPixel::Monochrome => todo!(),
                    BitmapBitsPerPixel::Palletize4Bit => todo!(),
                    BitmapBitsPerPixel::Palletize8Bit => {
                        let _ = self.output.write(
                            c.get_contents()
                                .iter()
                                .map(|c| c.intensity())
                                .collect::<Vec<u8>>()
                                .as_slice(),
                        );
                    }
                    BitmapBitsPerPixel::Rgb16Bit => todo!(),
                    BitmapBitsPerPixel::Rgb24Bit => {
                        let _ = self.output.write(
                            c.get_contents()
                                .iter()
                                .map(|c| [c.blue(), c.green(), c.red()])
                                .collect::<Vec<[u8; 3]>>()
                                .concat()
                                .as_slice(),
                        );
                    }
                }
            }
        }
    }
}
