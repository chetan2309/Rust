#[cfg(test)]
mod tests {
    use crate::parse_string::ParseString;

    #[test]
    fn test_parse_string_to_int_valid() {
        let input = "123";
        let expected_output = Ok(123);
        let actual_output = u32::parse_string(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_parse_string_to_int_invalid() {
        let input = "invalid input";
        let actual_output = u32::parse_string(input);
        assert!(actual_output.is_err());
    }

    #[test]
    fn test_parse_string_to_negative_integer() {
        let input = "-123";
        let actual_output = i32::parse_string(input);
        let expected_output = Ok(-123);
        assert_eq!(actual_output, expected_output);
    }
}