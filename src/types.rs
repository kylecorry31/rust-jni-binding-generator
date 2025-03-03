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
