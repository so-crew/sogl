extern crate sogl;

use std::io;

use sogl::display::{
    BitmapBitsPerPixel, BitmapCompression, BitmapOptions, ImageDisplayBuilder, ImageFormat,
};

#[cfg(test)]
mod image_displayer_builder_tests {

    use sogl::error::Error;

    use super::*;

    #[test]
    fn test_build_image_displayer_no_image_format() {
        let result = ImageDisplayBuilder::new().build().unwrap_err();
        assert!(matches!(result, Error::MissingParams(_)));
    }

    #[test]
    fn test_build_image_displayer_no_output() {
        let result = ImageDisplayBuilder::new()
            .set_image_format(ImageFormat::Bitmap(BitmapOptions {
                bits_per_pixel: BitmapBitsPerPixel::Rgb24Bit,
                compression: BitmapCompression::BIRGB,
                x_pixels_per_meter: i32::MAX as u32,
                y_pixels_per_meter: i32::MAX as u32,
            }))
            .build()
            .unwrap_err();

        assert!(matches!(result, Error::MissingParams(_)));
    }

    #[test]
    fn test_build_text_displayer_success() {
        let stream = &mut io::stdout().lock();
        let result = ImageDisplayBuilder::new()
            .set_image_format(ImageFormat::Bitmap(BitmapOptions {
                bits_per_pixel: BitmapBitsPerPixel::Rgb24Bit,
                compression: BitmapCompression::BIRGB,
                x_pixels_per_meter: i32::MAX as u32,
                y_pixels_per_meter: i32::MAX as u32,
            }))
            .set_output(stream)
            .build();

        assert!(matches!(result, Ok(_)));
    }
}
