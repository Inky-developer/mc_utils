use std::{
    path::Path,
    process::{Child, Command, Stdio},
};

pub fn run_server(path: impl AsRef<Path>, args: &[&str]) -> std::io::Result<Child> {
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
