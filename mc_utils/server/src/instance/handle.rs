use std::{
    path::Path,
    process::{Child, Command, Stdio},
};

thread_local! {
    static HAS_JAVA: bool = has_java();
}

/// Checks whether the java command is available on this system
pub fn has_java() -> bool {
    match Command::new("java").arg("-version").stdout(Stdio::null()).spawn() {
        Ok(mut child) => match child.wait() {
            Ok(status) => status.success(),
            _ => false,
        },
        _ => false,
    }
}

pub fn run_server(path: impl AsRef<Path>, args: &[&str]) -> std::io::Result<Child> {
    HAS_JAVA.with(|java| {
        if !java {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "The java command was not found on this system."))
        } else {
            Ok(())
        }
    })?;

    let path = path.as_ref();
    let dir = path
        .parent()
        .expect("Could not get parent dir of this path");

    Command::new("java")
        .arg("-jar")
        .arg(path)
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
}
