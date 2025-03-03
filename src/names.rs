pub fn to_pascal_case(name: &str) -> String {
    name.replace(['-', '_'], " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first
                    .to_uppercase()
                    .chain(chars.map(|c| c.to_lowercase().next().unwrap()))
                    .collect::<String>(),
            }
        })
        .collect::<String>()
}

pub fn to_camel_case(name: &str) -> String {
    let pascal = to_pascal_case(name);
    let mut chars = pascal.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().chain(chars).collect(),
    }
}

pub fn get_modules(full_path: &str) -> Vec<String> {
    let split = full_path.split("::").collect::<Vec<_>>();
    let count = split.len();
    split
        .iter()
        .take(count - 1)
        .map(|s| s.to_string())
        .collect()
}

pub fn get_unqualified_name(full_path: &str) -> String {
    full_path.split("::").last().unwrap().to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("hello-world", "HelloWorld")]
    #[case("foo_bar", "FooBar")]
    #[case("simple", "Simple")]
    #[case("ALREADY_UPPER", "AlreadyUpper")]
    fn test_to_pascal_case(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(to_pascal_case(input), expected);
    }

    #[rstest]
    #[case("hello-world", "helloWorld")]
    #[case("foo_bar", "fooBar")]
    #[case("simple", "simple")]
    #[case("ALREADY_UPPER", "alreadyUpper")]
    fn test_to_camel_case(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(to_camel_case(input), expected);
    }

    #[rstest]
    #[case("a::b::c", vec!["a", "b"])]
    #[case("x::y", vec!["x"])]
    #[case("single", vec![])]
    fn test_get_modules(#[case] input: &str, #[case] expected: Vec<&str>) {
        assert_eq!(get_modules(input), expected);
    }

    #[rstest]
    #[case("a::b::c", "c")]
    #[case("x::y", "y")]
    #[case("single", "single")]
    fn test_get_unqualified_name(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(get_unqualified_name(input), expected);
    }
}
