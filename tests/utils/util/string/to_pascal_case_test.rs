#[cfg(test)]
mod tests {
    use dechdev_rs::utils::string::to_pascal_case;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(
            to_pascal_case("this is a test string", true),
            "ThisIsATestString"
        );
        // assert_eq!(to_pascal_case("another test case", true), "AnotherTestCase");
        // assert_eq!(to_pascal_case("singleword", true), "Singleword");
        // assert_eq!(to_pascal_case("", true), "");
        // assert_eq!(
        //     to_pascal_case("  leading and trailing spaces  ", true),
        //     "LeadingAndTrailingSpaces"
        // );
    }

    #[test]
    fn test_to_pascal_case_with_special_characters() {
        assert_eq!(
            to_pascal_case("test string with underscores", true),
            "TestStringWithUnderscores"
        );
        assert_eq!(
            to_pascal_case("test string with dashes", true),
            "TestStringWithDashes"
        );
    }
}
