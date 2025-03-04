use std::collections::HashMap;

use crate::{
    config::Member,
    names::{get_modules, get_unqualified_name, to_camel_case, to_pascal_case},
    types::{convert_java_type_to_rust, convert_rust_type_to_java, get_jni_type, get_kotlin_type},
};

const JNI_FUNCTION_TEMPLATE: &str = r#"#[unsafe(no_mangle)]
pub extern "C" fn {name}(
    mut env: JNIEnv,
    _: JClass,
{params}
){ret_type} {
    {result_assignment}{func_name}({args});
    {return}
}"#;

const KOTLIN_FUNCTION_TEMPLATE: &str = r#"external fun {name}({params}){ret_type}"#;

fn populate_template(template: &str, parameters: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in parameters {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
}

fn get_jni_function_name(java_package: &str, name: &str) -> String {
    let module_names = get_modules(name)
        .iter()
        .map(|m| to_pascal_case(m))
        .collect::<Vec<_>>()
        .join("_");
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

    let params = HashMap::from([
        (
            "name".to_string(),
            get_jni_function_name(java_package, &function.name),
        ),
        ("params".to_string(), inputs.join(",\n")),
        (
            "ret_type".to_string(),
            output
                .as_ref()
                .map_or(String::new(), |o| format!(" -> {}", o)),
        ),
        (
            "result_assignment".to_string(),
            if output.is_some() {
                "let result = "
            } else {
                ""
            }
            .to_string(),
        ),
        ("func_name".to_string(), unqualified_name.to_string()),
        (
            "args".to_string(),
            function
                .inputs
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .map(|i| convert_java_type_to_rust(&i.name, &i.rust_type))
                .collect::<Vec<_>>()
                .join(", "),
        ),
        (
            "return".to_string(),
            if output.is_some() {
                convert_rust_type_to_java("result", function.output.as_ref().unwrap())
            } else {
                "".to_string()
            },
        ),
    ]);

    populate_template(JNI_FUNCTION_TEMPLATE, &params)
}

pub fn generate_kotlin_function(function: &Member) -> String {
    let inputs = function
        .inputs
        .as_ref()
        .unwrap_or(&vec![])
        .iter()
        .map(|i| format!("{}: {}", i.name, get_kotlin_type(&i.rust_type)))
        .collect::<Vec<_>>()
        .join(", ");

    let output = function
        .output
        .as_ref()
        .map(|t| format!(": {}", get_kotlin_type(t)))
        .unwrap_or_default();

    // let unqualified_name = function.name.split("::").last().unwrap();
    let module_names = get_modules(&function.name)
        .iter()
        .skip(1)
        .map(|m| to_pascal_case(m))
        .collect::<Vec<_>>()
        .join("_");
    let function_name = to_camel_case(&get_unqualified_name(&function.name));

    let name = format!("{}_{}", module_names, function_name);

    let params = HashMap::from([
        ("name".to_string(), name.clone()),
        ("params".to_string(), inputs),
        ("ret_type".to_string(), output.clone()),
        ("func_name".to_string(), name),
    ]);

    populate_template(KOTLIN_FUNCTION_TEMPLATE, &params)
}

#[cfg(test)]
mod tests {
    use crate::config::Input;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("com.example", "test::func", "Java_com_example_Test_func")]
    #[case("com.example_test", "test::func", "Java_com_example_1test_Test_func")]
    #[case("com.example", "mod1::mod2::func", "Java_com_example_Mod1_Mod2_func")]
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
pub extern "C" fn Java_com_example_Test_func(
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
