use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Blocks {
    #[serde(flatten)]
    pub blocks: HashMap<String, Block>,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    #[serde(default)]
    pub properties: HashMap<String, Vec<String>>,
    pub states: Vec<BlockState>,
}

#[derive(Debug, Deserialize)]
pub struct BlockState {
    pub id: u32,
    #[serde(default)]
    pub properties: HashMap<String, String>,
}
