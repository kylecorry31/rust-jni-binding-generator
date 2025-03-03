use crate::{
    config::Member,
    names::{get_modules, get_unqualified_name, to_camel_case},
    types::get_jni_type,
};

fn get_jni_function_name(java_package: &str, name: &str) -> String {
    let module_names = get_modules(name).join("_");
    let function_name = to_camel_case(&get_unqualified_name(name));
    format!(
        "Java_{}_{}_{}",
        java_package.replace('_', "_1").replace('.', "_"),
        module_names,
        function_name
    )
}

pub fn generate_jni_function(java_package: &str, function: &Member) -> String {
    let mut inputs = Vec::new();

    for arg in function.inputs.as_ref().unwrap_or(&vec![]) {
        inputs.push(format!(
            "    {}: {}",
            arg.name,
            get_jni_type(&arg.rust_type)
        ));
    }

    let output = function.output.as_ref().map(|o| get_jni_type(o));

    let unqualified_name = function.name.split("::").last().unwrap();

    format!(
        "#[unsafe(no_mangle)]\npub extern \"C\" fn {}(\n    mut env: JNIEnv,\n    _: JClass,\n{}\n){} {{\n    {}{}({});\n    {}\n}}",
        get_jni_function_name(java_package, &function.name),
        inputs.join(",\n"),
        output
            .as_ref()
            .map_or(String::new(), |o| format!(" -> {}", o)),
        if output.is_some() {
            "let result = "
        } else {
            ""
        },
        unqualified_name,
        function
            .inputs
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|i| i.name.as_ref())
            .collect::<Vec<_>>()
            .join(", "),
        if output.is_some() { "result" } else { "" }
    )
}

#[cfg(test)]
mod tests {
    use crate::config::Input;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("com.example", "test::func", "Java_com_example_test_func")]
    #[case("com.example_test", "test::func", "Java_com_example_1test_test_func")]
    #[case("com.example", "mod1::mod2::func", "Java_com_example_mod1_mod2_func")]
    fn test_jni_function_name(#[case] package: &str, #[case] name: &str, #[case] expected: &str) {
        assert_eq!(get_jni_function_name(package, name), expected);
    }

    #[test]
    fn test_generate_jni_function() {
        let function = Member {
            member_type: "function".to_string(),
            name: "test::func".to_string(),
            inputs: Some(vec![Input {
                name: "arg1".to_string(),
                rust_type: "i32".to_string(),
            }]),
            output: Some("bool".to_string()),
        };

        let expected = r#"#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_test_func(
    mut env: JNIEnv,
    _: JClass,
    arg1: jint
) -> jboolean {
    let result = func(arg1);
    result
}"#;

        assert_eq!(generate_jni_function("com.example", &function), expected);
    }
}
