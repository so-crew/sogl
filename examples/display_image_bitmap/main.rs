use std::fs::File;

use sogl::{
    display::{Canvas, CanvasCoordinate, Displayer, ImageDisplayBuilder, DEFAULT_IMAGE_FORMAT},
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

    let file = File::create("out.bmp").unwrap();

    let mut displayer = ImageDisplayBuilder::new()
        .set_image_format(DEFAULT_IMAGE_FORMAT)
        .set_output(file)
        .build()
        .unwrap();

    displayer.show(&subject);
}
