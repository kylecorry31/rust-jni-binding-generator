use json::JsonValue;

use crate::{
    casing::{to_camel_case, to_pascal_case},
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
    let module_names = name
        .split("::")
        .take(name.split("::").count() - 1)
        .map(to_pascal_case)
        .collect::<Vec<_>>()
        .join("_");

    let function_name = to_camel_case(name.split("::").last().unwrap());

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
        _ => return None,
    };
    let output = match &element["output"] {
        JsonValue::String(s) => Some(s.clone()),
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
            .map_or(String::new(), |o| format!(" -> {} ", o)),
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
