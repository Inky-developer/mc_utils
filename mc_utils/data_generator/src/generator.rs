use crate::data::GeneratedData;
use server::{download_server, run_server, VersionInfo};
use std::env::temp_dir;
use std::path::{Path, PathBuf};

pub fn generate_reports_for_version(version: &VersionInfo) -> std::io::Result<GeneratedData> {
    let temp_dir = temp_dir().join("minecraft_server");
    std::fs::create_dir(&temp_dir)?;

    let server_jar_path = temp_dir.join("server.jar");
    download_server(version, &server_jar_path)?;

    let reports_dir = generate_reports(server_jar_path)?;

    let data = GeneratedData::from_reports_dir(reports_dir)?;

    std::fs::remove_dir_all(temp_dir)?;

    Ok(data)
}

/// Generates reports using the given server and returns the path to the reports directory
pub fn generate_reports(server_jar: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    let mut process = run_server(
        &server_jar,
        &["--reports"],
        &["-DbundlerMainClass=net.minecraft.data.Main"],
    )?;
    process.wait()?;
    let report_dir = server_jar
        .as_ref()
        .parent()
        .unwrap()
        .join("generated/reports");
    Ok(report_dir)
}
