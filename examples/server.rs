//! Downloads and runs a custom server

use std::{env::temp_dir, fs::create_dir, fs::remove_dir_all, path::PathBuf};

use server::{download_file, ServerInstance, VersionManifest};

fn readline(prompt: &str) -> String {
    use std::io::Write;

    print!("{}", prompt);
    std::io::stdout().flush().expect("Failed to flush stdout");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Could not read line");
    line.trim_end().to_string()
}

fn download_server() -> PathBuf {
    let version_manifest = VersionManifest::default();
    let latest_version = version_manifest
        .find_version(version_manifest.latest_snapshot())
        .expect("Could not find latest version");
    let jar_url = latest_version.jar_url().expect("Could not find url");

    let server_jar = temp_dir().join("minecraft_server/server.jar");
    create_dir(server_jar.parent().unwrap()).ok();
    download_file(jar_url, server_jar.clone()).expect("Could not download server");
    server_jar
}

fn main() {
    let server_jar = download_server();
    println!("Downloaded server to: {}", server_jar.to_str().unwrap());

    let mut server = ServerInstance::with_jar(server_jar)
        .build()
        .expect("Could not start server");

    loop {
        let input = readline("> ");
        if input.is_empty() {
            break;
        }

        server.command(&input).expect("Could not send command");
    }

    server
        .try_stop()
        .expect("Could not gracefully stop the server");
    remove_dir_all(server.dir()).expect("Could not remove the temporary directory");
}
