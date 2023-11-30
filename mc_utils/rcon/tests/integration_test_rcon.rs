use rcon::McRcon;
use server::{download_server, ServerInstance, VersionManifest};
use tempfile::{tempdir, TempDir};

pub const RCON_PORT: u16 = 25575;
pub const RCON_PASSWORD: &str = "1234";

fn setup() -> (TempDir, ServerInstance, McRcon) {
    let dir = tempdir().expect("Could not create a temporary directory");

    let version_manifest = VersionManifest::default();
    let latest_version = version_manifest
        .find_version(version_manifest.latest_snapshot())
        .expect("Could not find latest version");

    download_server(&latest_version, dir.path().join("server.jar"));

    let server = ServerInstance::builder(dir.path())
        .property("enable-rcon", "true")
        .property("rcon.password", RCON_PASSWORD)
        .property("rcon.port", RCON_PORT.to_string())
        .build()
        .expect("Could not start server");

    let rcon = McRcon::new(("localhost", RCON_PORT), RCON_PASSWORD.to_string())
        .expect("Could not connect rcon");

    (dir, server, rcon)
}

#[test]
fn test_rcon() {
    let (_tempdir, mut server, rcon) = setup();

    run_tests(rcon);

    server.try_stop().ok();
}

fn run_tests(mut rcon: McRcon) {
    rcon.command("weather rain")
        .expect("Failed to run command 'weather rain'");
    rcon.command("weather clear")
        .expect("Failed to run command 'weather clear'");

    let players = rcon.command("execute if entity @a");
    assert!(players.is_ok());
}
