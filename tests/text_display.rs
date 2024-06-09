extern crate sogl;

use sogl::display::{TextDisplayBuilder, DEFAULT_CHARSET};
use std::io;

#[cfg(test)]
mod text_displayer_tests {
    use sogl::model::Color;

    use super::*;

    #[test]
    fn test_color_to_char() {
        let dark = Color::new(0, 0, 0, u8::MAX);
        let medium = Color::new(120, 120, 120, u8::MAX);
        let light = Color::new(u8::MAX, u8::MAX, u8::MAX, u8::MAX);

        let stream = io::stdout();
        let displayer = TextDisplayBuilder::new()
            .set_charset("123")
            .set_output(stream)
            .build()
            .unwrap();

        assert_eq!('1', displayer.color_to_char(&dark));
        assert_eq!('2', displayer.color_to_char(&medium));
        assert_eq!('3', displayer.color_to_char(&light));
    }
}

#[cfg(test)]
mod text_displayer_builder_tests {
    use std::fs::{self, File};

    use sogl::error::Error;

    use super::*;

    #[test]
    fn test_build_text_displayer_no_charset() {
        let result = TextDisplayBuilder::new().build().unwrap_err();
        assert!(matches!(result, Error::MissingParams(_)));
    }

    #[test]
    fn test_build_text_displayer_no_output() {
        let result = TextDisplayBuilder::new()
            .set_charset(&DEFAULT_CHARSET)
            .build()
            .unwrap_err();
        assert!(matches!(result, Error::MissingParams(_)));
    }

    #[test]
    fn test_build_default_text_displayer_success() {
        let result = TextDisplayBuilder::default().build();
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_build_stdout_text_displayer_success() {
        let stream = io::stdout();
        let result = TextDisplayBuilder::new()
            .set_charset(&DEFAULT_CHARSET)
            .set_output(stream)
            .build();

        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_build_file_text_displayer_success() {
        let stream = File::create("test.txt").unwrap();
        let result = TextDisplayBuilder::new()
            .set_charset(&DEFAULT_CHARSET)
            .set_output(stream)
            .build();
        let _ = fs::remove_file("test.txt");

        assert!(matches!(result, Ok(_)));
    }
}
