use std::num::ParseIntError;
pub(crate) fn parse_string(input: &str) -> Result<u32, ParseIntError> {
    input.trim().parse()
}