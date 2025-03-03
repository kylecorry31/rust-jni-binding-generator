use json::JsonValue;

use crate::{
    names::{get_modules, get_unqualified_name, to_camel_case},
    types::get_jni_type,
};

pub struct RustFunctionInput {
    pub name: String,
    pub rust_type: String,
}

pub struct RustFunction {
    pub name: String,
    pub inputs: Vec<RustFunctionInput>,
    pub output: Option<String>,
}

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

pub fn parse_jni_function_json(element: &JsonValue) -> Option<RustFunction> {
    let inputs_json = element["inputs"].members().collect::<Vec<_>>();
    let inputs: Vec<_> = inputs_json
        .iter()
        .map(|i| RustFunctionInput {
            name: i["name"].to_string(),
            rust_type: i["type"].to_string(),
        })
        .collect();
    let name = match &element["name"] {
        JsonValue::String(s) => s.clone(),
        JsonValue::Short(s) => s.to_string(),
        _ => return None,
    };
    let output = match &element["output"] {
        JsonValue::String(s) => Some(s.clone()),
        JsonValue::Short(s) => Some(s.to_string()),
        _ => None,
    };
    Some(RustFunction {
        name,
        inputs,
        output,
    })
}

pub fn generate_jni_function(java_package: &str, function: &RustFunction) -> String {
    let mut inputs = Vec::new();

    for arg in &function.inputs {
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
            .iter()
            .map(|i| i.name.as_ref())
            .collect::<Vec<_>>()
            .join(", "),
        if output.is_some() { "result" } else { "" }
    )
}

#[cfg(test)]
mod tests {
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
    fn test_parse_jni_function() {
        let json = json::parse(
            r#"{
            "name": "test::func",
            "type": "function",
            "inputs": [
                {"name": "arg1", "type": "i32"},
                {"name": "arg2", "type": "f64"}
            ],
            "output": "bool"
        }"#,
        )
        .unwrap();

        let function = parse_jni_function_json(&json).unwrap();
        assert_eq!(function.name, "test::func");
        assert_eq!(function.inputs.len(), 2);
        assert_eq!(function.inputs[0].name, "arg1");
        assert_eq!(function.inputs[0].rust_type, "i32");
        assert_eq!(function.inputs[1].name, "arg2");
        assert_eq!(function.inputs[1].rust_type, "f64");
        assert_eq!(function.output, Some("bool".to_string()));
    }

    #[test]
    fn test_generate_jni_function() {
        let function = RustFunction {
            name: "test::func".to_string(),
            inputs: vec![RustFunctionInput {
                name: "arg1".to_string(),
                rust_type: "i32".to_string(),
            }],
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
