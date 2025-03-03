pub fn get_jni_type(rust_type: &str) -> String {
    let jtype = match rust_type {
        "i8" => "jbyte",
        "i16" => "jshort",
        "i32" => "jint",
        "i64" => "jlong",
        "f32" => "jfloat",
        "f64" => "jdouble",
        "bool" => "jboolean",
        "char" => "jchar",
        // TODO: Handle arrays, strings, and objects
        _ => panic!("Unsupported type: {}", rust_type),
    };
    String::from(jtype)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("i8", "jbyte")]
    #[case("i16", "jshort")]
    #[case("i32", "jint")]
    #[case("i64", "jlong")]
    #[case("f32", "jfloat")]
    #[case("f64", "jdouble")]
    #[case("bool", "jboolean")]
    #[case("char", "jchar")]
    fn test_get_jni_type(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(get_jni_type(input), expected);
    }

    #[rstest]
    #[should_panic(expected = "Unsupported type: unsupported")]
    fn test_get_jni_type_unsupported() {
        get_jni_type("unsupported");
    }
}
