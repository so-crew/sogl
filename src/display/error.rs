use crate::error::Error;

pub const CHARSET_NOT_SET_ERROR: Error = Error{message: "charset not set"};
pub const OUTPUT_NOT_SET_ERROR: Error = Error{message: "output not set"};