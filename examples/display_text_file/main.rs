extern crate sogl;

use std::fs::File;

use sogl::display::{Canvas, CanvasCoordinate, Displayer, TextDisplayBuilder, DEFAULT_CHARSET};
use sogl::model::Color;

fn main() {
    let size = 100;

    let mut subject = Canvas::new(size, size);

    for i in 0..size.pow(2) {
        let val = (i * u8::MAX as usize / size) as u8;
        let _ = subject.set_content(
            CanvasCoordinate::Linear(i),
            &Color::new(val, val, val, u8::MAX),
        );
    }

    let file = &mut File::create("./out.txt").unwrap();
    let mut displayer = TextDisplayBuilder::new()
        .set_charset(DEFAULT_CHARSET)
        .set_output(file)
        .build()
        .unwrap();

    displayer.show(&subject);
}
