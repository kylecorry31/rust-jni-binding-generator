use config::parse;
use functions::generate_jni_function;
use std::fs::File;
use std::io::Write;
use std::{env, fs, path::Path};

mod cargo;
mod config;
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
    let config = parse(&fs::read_to_string(config_path)?).unwrap();

    // Create cargo lib
    fs::create_dir_all(codegen_path)?;
    cargo::new_lib(lib_name, codegen_path)?;

    // Add dependencies
    for package in config.iter() {
        cargo::add(&package.name, lib_path.to_str().unwrap())?;
    }

    cargo::add("jni", lib_path.to_str().unwrap())?;

    // Generate lib.rs
    let mut imports = vec![
        "jni::objects::{JClass, JString, JObject}".to_string(),
        "jni::sys::{jfloat, jstring, jdouble, jint, jlong, jbyte, jshort, jchar, jboolean}"
            .to_string(),
        "jni::JNIEnv".to_string(),
    ];

    let mut bindings = Vec::new();

    for package in config.iter() {
        for member in &package.members {
            imports.push(member.name.clone());
            if member.member_type == "function" {
                bindings.push(generate_jni_function(java_package, member));
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
    cargo::clippy_fix(lib_path.to_str().unwrap())?;
    cargo::format(lib_path.to_str().unwrap())?;

    Ok(())
}
