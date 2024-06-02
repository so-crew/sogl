extern crate sogl;

use sogl::display::{
    TextDisplayBuilder, DEFAULT_CHARSET, ERROR_CHARSET_NOT_SET, ERROR_OUTPUT_NOT_SET,
};
use std::io;

#[cfg(test)]
mod text_displayer_tests {
    use sogl::model::Color;

    use super::*;

    #[test]
    fn test_build_text_displayer_no_charset() {
        let dark = Color::new(0, 0, 0, u8::MAX);
        let medium = Color::new(120, 120, 120, u8::MAX);
        let light = Color::new(u8::MAX, u8::MAX, u8::MAX, u8::MAX);

        let mut stream = io::stdout();
        let displayer = TextDisplayBuilder::new()
            .set_charset("123")
            .set_output_stream(&mut stream)
            .build()
            .unwrap();

        assert_eq!('1', displayer.color_to_char(&dark));
        assert_eq!('2', displayer.color_to_char(&medium));
        assert_eq!('3', displayer.color_to_char(&light));
    }
}

#[cfg(test)]
mod text_displayer_builder_tests {
    use super::*;

    #[test]
    fn test_build_text_displayer_no_charset() {
        let result = TextDisplayBuilder::new().build();
        assert!(matches!(result, Err(ERROR_CHARSET_NOT_SET)));
    }

    #[test]
    fn test_build_text_displayer_no_output() {
        let result = TextDisplayBuilder::new()
            .set_charset(&DEFAULT_CHARSET)
            .build();
        assert!(matches!(result, Err(ERROR_OUTPUT_NOT_SET)));
    }

    #[test]
    fn test_build_text_displayer_success() {
        let mut stream = io::stdout();
        let result = TextDisplayBuilder::new()
            .set_charset(&DEFAULT_CHARSET)
            .set_output_stream(&mut stream)
            .build();

        assert!(matches!(result, Ok(_)));
    }
}
