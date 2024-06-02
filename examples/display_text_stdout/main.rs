extern crate sogl;

use std::io;

use sogl::display::{Canvas, TextDisplayBuilder};
use sogl::display::Displayer;
use sogl::model::Color;

fn main() {
    let size = usize::from(u8::MAX);
    
    let mut subject = Canvas::new(size, size);

    for _ in 0..size {
        for i in 0..size {
            let val = (i % u8::MAX as usize) as u8;
            subject.content.push(Color::new(val, val, val, u8::MAX))
        }
    }
    
    let mut stream = io::stdout().lock();
    let stream: Box<&mut dyn io::Write> = Box::new(&mut stream);

    let displayer = TextDisplayBuilder::default()
        .set_output_stream(stream)
        .build();

    displayer.show(&subject);
}
