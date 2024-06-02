use std::fs::File;

use sogl::{
    display::{
        BitmapBitsPerPixel, BitmapOptions, Canvas, CanvasCoordinate, Displayer,
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

    let displayer = ImageDisplayBuilder::new()
        .set_image_format(ImageFormat::Bitmap(BitmapOptions {
            bits_per_pixel: BitmapBitsPerPixel::Rgb24Bit,
            compression: 0,
            x_pixels_per_meter: u16::MAX as u32,
            y_pixels_per_meter: u16::MAX as u32,
            colors_used: 0,
            important_colors: 0,
        }))
        .set_output(file)
        .build()
        .unwrap();

    displayer.show(&subject);
}
