#[cfg(test)]
mod tests {

    use dechdev_rs::utils::string::to_camel_case;

    #[test]
    fn test_empty_string() {
        assert_eq!(to_camel_case("", true), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(to_camel_case("hello", true), "hello");
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(to_camel_case("hello world", true), "helloWorld");
    }
}
