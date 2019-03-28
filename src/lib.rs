pub mod parseargs;
pub mod guessing;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsing() {
        let test_input = String::from("1 2 3 4");
        let test_result: Vec<i64> = match parseargs::parse_args(4, &test_input) {
            Ok(result) => result,
            Err(e) => panic!("{}", e),
        };

        let test_should_eq: Vec<i64> = vec![1, 2, 3, 4];
        assert_eq!(test_result, test_should_eq);
    }

    #[test]
    #[should_panic]
    fn parsearg_invalid_input() {
        let test_input = "1 2 3 4";
        let _test_result: Vec<i64> = match parseargs::parse_args(3, &test_input) {
            Ok(result) => result,
            Err(e) => panic!("{}", e),
        };
    }

    #[test]
    #[should_panic]
    fn parsearg_nan() {
        let test_input = "1 2 hobo 4";
        let _test_result: Vec<i64> = match parseargs::parse_args(4, &test_input) {
            Ok(result) => result,
            Err(e) => panic!("{}", e),
        };
    }
}
