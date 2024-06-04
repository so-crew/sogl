extern crate sogl;

use std::io;

use sogl::display::{Canvas, CanvasCoordinate, Displayer, TextDisplayBuilder, DEFAULT_CHARSET};
use sogl::model::Color;

fn main() {
    let size: usize = 100;

    let mut subject = Canvas::new(size, size);

    for j in 0..size {
        for i in 0..size {
            let val = (i * u8::MAX as usize / size) as u8;
            let _ = subject.set_content(
                CanvasCoordinate::Cartesian(j, i),
                &Color::new(val, val, val, u8::MAX),
            );
        }
    }

    let mut stream = io::stdout().lock();

    let displayer = TextDisplayBuilder::new()
        .set_charset(DEFAULT_CHARSET)
        .set_output_stream(&mut stream)
        .build()
        .unwrap();

    displayer.show(&subject);
}
