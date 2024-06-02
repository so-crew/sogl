use std::fs::File;

use sogl::{
    display::{
        BitmapBitsPerPixel, BitmapCompression, BitmapOptions, Canvas, CanvasCoordinate, Displayer,
        ImageDisplayBuilder, ImageFormat,
    },
    model::Color,
};

extern crate sogl;

fn main() {
    let size = 256;

    let mut subject = Canvas::new(size, size);

    for i in 0..size.pow(2) {
        let val = (i % 256) as u8;
        let _ = subject.set_content(CanvasCoordinate::Linear(i), &Color::new(val, 0, 0, u8::MAX));
    }

    let file = &mut File::create("out.bmp").unwrap();

    let mut displayer = ImageDisplayBuilder::new()
        .set_image_format(ImageFormat::Bitmap(BitmapOptions {
            bits_per_pixel: BitmapBitsPerPixel::Rgb24Bit,
            compression: BitmapCompression::BIRGB,
            x_pixels_per_meter: u16::MAX as u32,
            y_pixels_per_meter: u16::MAX as u32,
        }))
        .set_output(file)
        .build()
        .unwrap();

    displayer.show(&subject);
}
