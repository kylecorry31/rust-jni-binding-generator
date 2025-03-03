use std::{fs::OpenOptions, io::Result, io::Write, process::Command};

pub fn add(_crate: &str, path: &str) -> Result<()> {
    Command::new("cargo")
        .args(["add", _crate])
        .current_dir(path)
        .status()?;
    Ok(())
}

pub fn new_lib(name: &str, path: &str) -> Result<()> {
    Command::new("cargo")
        .args(["new", "--lib", name])
        .current_dir(path)
        .status()?;

    let mut file = OpenOptions::new()
        .append(true)
        .open(format!("{}/{}/Cargo.toml", path, name))?;

    writeln!(file, "\n[lib]")?;
    writeln!(file, "crate-type = [\"cdylib\"]")?;

    Ok(())
}

pub fn clippy_fix(path: &str) -> Result<()> {
    Command::new("cargo")
        .args(["clippy", "--fix", "--allow-dirty"])
        .current_dir(path)
        .status()?;
    Ok(())
}

pub fn format(path: &str) -> Result<()> {
    Command::new("cargo")
        .args(["fmt"])
        .current_dir(path)
        .status()?;
    Ok(())
}
