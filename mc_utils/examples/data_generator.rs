use data_generator::generate_reports_for_version;
use server::VersionManifest;

fn main() {
    let manifest = VersionManifest::default();
    let latest_snapshot = manifest.find_version(manifest.latest_snapshot()).unwrap();
    let data = generate_reports_for_version(latest_snapshot).unwrap();

    println!("{data:?}");
}
