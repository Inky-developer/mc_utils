use std::{cmp::Ordering, path::Path};
use std::{fs::File, io};

use chrono::DateTime;
use reqwest::Url;
use serde::{Deserialize, Deserializer};

pub const VERSION_MANIFEST_URL: &'static str =
    "https://launchermeta.mojang.com/mc/game/version_manifest.json";

/// Downloads a file from 'url' to the file at 'destination'
///
/// On success, the total number of bytes is returned
pub fn download_file(url: &str, destination: impl AsRef<Path>) -> io::Result<u64> {
    let mut response =
        reqwest::blocking::get(url).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    let mut out = File::create(destination.as_ref())?;

    Ok(response
        .copy_to(&mut out)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?)
}

#[derive(Debug, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    Snapshot,
    Release,
    OldBeta,
    OldAlpha,
}

#[derive(Debug, Deserialize, Eq)]
pub struct VersionInfo {
    #[serde(rename(deserialize = "id"))]
    name: String,
    #[serde(rename(deserialize = "type"))]
    typ: VersionType,
    url: String,
    /// The release time is used to uniquely identify a version
    #[serde(
        rename(deserialize = "releaseTime"),
        deserialize_with = "deserialize_time"
    )]
    release_time: i64,
}

impl VersionInfo {
    pub fn jar_url(&self) -> Option<Url> {
        let data: serde_json::Value = reqwest::blocking::get(&self.url).ok()?.json().ok()?;

        Some(Url::parse(data.get("downloads")?.get("server")?.get("url")?.as_str()?).ok()?)
    }
}

// Order and equality of Versions depend on their release time.
// A "greater" version was release later

impl PartialEq for VersionInfo {
    fn eq(&self, other: &VersionInfo) -> bool {
        self.release_time == other.release_time
    }
}

impl PartialOrd for VersionInfo {
    fn partial_cmp(&self, other: &VersionInfo) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.release_time.cmp(&other.release_time)
    }
}

#[derive(Debug, Deserialize)]
pub struct LatestVerions {
    release: String,
    snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct VersionManifest {
    latest: LatestVerions,
    /// A sorted vector of versions, the latest version is at index 0
    versions: Vec<VersionInfo>,
}

impl VersionManifest {
    /// Searches linearly for a version with 'name'
    /// 
    /// Starts at the latest version
    pub fn find_version(&self, name: &str) -> Option<&VersionInfo> {
        self.versions.iter().find(|info| info.name == name)
    }

    pub fn latest_release(&self) -> &str {
        &self.latest.release
    }

    pub fn latest_snapshot(&self) -> &str {
        &self.latest.snapshot
    }
}

impl Default for VersionManifest {
    fn default() -> Self {
        let mut manifest: VersionManifest = reqwest::blocking::get(VERSION_MANIFEST_URL)
            .expect("Could not download the version manifest")
            .json()
            .expect("Malformed response");

        manifest.versions.sort_unstable_by(|a, b| b.cmp(a));

        manifest
    }
}

fn deserialize_time<'de, D>(de: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let time_str = String::deserialize(de)?;
    Ok(DateTime::parse_from_rfc3339(&time_str)
        .map_err(|_| D::Error::custom("Could not parse timestamp"))?
        .timestamp())
}

#[cfg(test)]
mod test {
    use super::{download_file, VersionManifest};

    #[test]
    fn test_version_manifest() {
        let _manifest = VersionManifest::default();
    }

    #[test]
    fn test_get_specific_version() {
        let manifest = VersionManifest::default();

        let info = manifest.find_version(manifest.latest_release());
        assert!(info.is_some())
    }

    #[test]
    fn test_get_version_jar_url() {
        let manifest = VersionManifest::default();

        let url = manifest
            .find_version("20w48a")
            .expect("Failed to find this version")
            .jar_url()
            .expect("Failed to find the version server jar");
        assert_eq!(url.as_str(), "https://launcher.mojang.com/v1/objects/a14d24f89d5a4ec7521b91909caf4fee89c997f4/server.jar")
    }

    #[test]
    fn test_download_server() {
        let manifest = VersionManifest::default();

        let url = manifest
            .find_version(manifest.latest_snapshot())
            .unwrap()
            .jar_url()
            .unwrap();
        let result = download_file(url.as_str(), "server.jar");

        assert!(result.is_ok());

        std::fs::remove_file("server.jar").expect("Could not remove file");
    }
}
