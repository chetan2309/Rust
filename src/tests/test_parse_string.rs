#[cfg(test)]
mod tests {
    use crate::parse_string::parse_string;

    #[test]
    fn test_parse_string_to_int_valid() {
        let input = "123";
        let expected_output = Ok(123);
        let actual_output = parse_string(input);
        assert_eq!(actual_output, expected_output);
    }
}