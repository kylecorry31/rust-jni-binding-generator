// use crate::converters::get_converter;

pub fn get_jni_type(rust_type: &str) -> String {
    // return get_converter(rust_type).get_jni_type();

    let jtype = match rust_type {
        "i8" => "jbyte",
        "i16" => "jshort",
        "i32" => "jint",
        "i64" => "jlong",
        "u8" => "jbyte",
        "u16" => "jshort",
        "u32" => "jint",
        "u64" => "jlong",
        "f32" => "jfloat",
        "f64" => "jdouble",
        "bool" => "jboolean",
        "char" => "jchar",
        // TODO: Handle arrays, strings, and objects
        _ => return rust_type.to_string(),
    };
    String::from(jtype)
}

pub fn convert_java_type_to_rust(name: &str, rust_type: &str) -> String {
    // unsigned values require .try_into().unwrap(), all others can pass through for now
    let conversion = match rust_type {
        "u8" | "u16" | "u32" | "u64" => format!("{}.try_into().unwrap()", name),
        _ => name.to_string(),
    };
    conversion
}

pub fn convert_rust_type_to_java(name: &str, rust_type: &str) -> String {
    // unsigned values require .try_into().unwrap(), all others can pass through for now
    let conversion = match rust_type {
        "u8" | "u16" | "u32" | "u64" => format!("{}.try_into().unwrap()", name),
        _ => name.to_string(),
    };
    conversion
}

pub fn get_kotlin_type(rust_type: &str) -> String {
    let kotlin_type = match rust_type {
        "i8" => "Byte",
        "i16" => "Short",
        "i32" => "Int",
        "i64" => "Long",
        "u8" => "Byte",
        "u16" => "Short",
        "u32" => "Int",
        "u64" => "Long",
        "f32" => "Float",
        "f64" => "Double",
        "bool" => "Boolean",
        "char" => "Char",
        // TODO: Handle arrays, strings, and objects
        _ => panic!("Unsupported type: {}", rust_type),
    };
    String::from(kotlin_type)
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
    #[case("u8", "jbyte")]
    #[case("u16", "jshort")]
    #[case("u32", "jint")]
    #[case("u64", "jlong")]
    #[case("f32", "jfloat")]
    #[case("f64", "jdouble")]
    #[case("bool", "jboolean")]
    #[case("char", "jchar")]
    fn test_get_jni_type(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(get_jni_type(input), expected);
    }

    #[rstest]
    #[case("i8", "Byte")]
    #[case("i16", "Short")]
    #[case("i32", "Int")]
    #[case("i64", "Long")]
    #[case("u8", "Byte")]
    #[case("u16", "Short")]
    #[case("u32", "Int")]
    #[case("u64", "Long")]
    #[case("f32", "Float")]
    #[case("f64", "Double")]
    #[case("bool", "Boolean")]
    #[case("char", "Char")]
    fn test_get_kotlin_type(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(get_kotlin_type(input), expected);
    }

    #[rstest]
    #[should_panic(expected = "Unsupported type: unsupported")]
    fn test_get_jni_type_unsupported() {
        get_jni_type("unsupported");
    }
}
