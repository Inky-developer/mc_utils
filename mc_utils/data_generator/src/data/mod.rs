use std::fs::File;
use std::path::Path;

pub mod blocks;

#[derive(Debug)]
pub struct GeneratedData {
    pub blocks: blocks::Blocks,
}

impl GeneratedData {
    pub fn from_reports_dir(dir: impl AsRef<Path>) -> std::io::Result<Self> {
        let blocks_file = File::open(dir.as_ref().join("blocks.json"))?;
        let blocks = serde_json::from_reader(blocks_file)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
        Ok(GeneratedData { blocks })
    }
}
