use std::num::ParseIntError;

/// A trait for parsing strings into a specific type.
pub(crate) trait ParseString {
    type Output;
    /// Parses a string into a value of the `Output` type.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to be parsed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed value wrapped in `Ok`, or a `ParseIntError` wrapped in `Err` if the string cannot be parsed.
    fn parse_string(s: &str) -> Result<Self::Output, ParseIntError>;
}

pub mod parse_strings {
    use std::num::ParseIntError;

    pub(crate) use super::ParseString;
    impl ParseString for u32 {
        type Output = u32;
        fn parse_string(s: &str) -> Result<Self::Output, ParseIntError> {
            s.trim().parse()
        }
    }
}