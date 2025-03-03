use functions::{generate_jni_function, parse_jni_function_json};
use json::JsonValue;
use std::fs::File;
use std::io::Write;
use std::{env, fs, path::Path, process::Command};

mod functions;
mod names;
mod types;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <lib> <config> <java_package>", args[0]);
        std::process::exit(1);
    }

    let lib_name = &args[1];
    let config_path = &args[2];
    let java_package = &args[3];

    let codegen_path = "codegen";

    // Clean existing generated code
    let lib_path = Path::new(codegen_path).join(lib_name);
    if lib_path.exists() {
        fs::remove_dir_all(&lib_path)?;
    }

    // Read config file
    let config: JsonValue = json::parse(&fs::read_to_string(config_path)?).unwrap();

    // Create cargo lib
    fs::create_dir_all(codegen_path)?;
    Command::new("cargo")
        .args(["new", "--lib", lib_name])
        .current_dir(codegen_path)
        .status()?;

    // Add dependencies
    for package in config.entries() {
        Command::new("cargo")
            .args(["add", package.0])
            .current_dir(&lib_path)
            .status()?;
    }

    Command::new("cargo")
        .args(["add", "jni"])
        .current_dir(&lib_path)
        .status()?;

    // Generate lib.rs
    let mut imports = vec![
        "jni::objects::{JClass, JString, JObject}".to_string(),
        "jni::sys::{jfloat, jstring, jdouble, jint, jlong, jbyte, jshort, jchar, jboolean}"
            .to_string(),
        "jni::JNIEnv".to_string(),
    ];

    let mut bindings = Vec::new();

    for (_, package) in config.entries() {
        for member in package["members"].members() {
            imports.push(member["name"].as_str().unwrap().to_string());
            if member["type"] == "function" {
                let parsed = parse_jni_function_json(member).expect("Invalid function definition");
                bindings.push(generate_jni_function(java_package, &parsed));
            }
        }
    }

    imports.sort();
    imports.dedup();

    let contents = format!(
        "{}\n\n{}\n",
        imports
            .iter()
            .map(|i| format!("use {};", i))
            .collect::<Vec<_>>()
            .join("\n"),
        bindings.join("\n\n")
    );

    let mut file = File::create(lib_path.join("src/lib.rs"))?;
    file.write_all(contents.as_bytes())?;

    // Format code
    Command::new("cargo")
        .args(["clippy", "--fix", "--allow-dirty"])
        .current_dir(&lib_path)
        .status()?;

    Command::new("cargo")
        .args(["fmt"])
        .current_dir(&lib_path)
        .status()?;

    Ok(())
}
