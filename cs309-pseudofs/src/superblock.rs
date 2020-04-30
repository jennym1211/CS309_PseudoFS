use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Superblock {
    pub magic_number: String, // 0x70736575646F4653 magic number
    pub total_blocks: u32,
    pub free_blocks: Vec<u32>,
    pub total_inodes: u32,
    pub free_inodes: Vec<u32>,
}

impl Superblock {
    pub fn toJSON(&self) {
        let serialized_block = serde_json::to_string(&self).unwrap();

        println!("{}", serialized_block);
    }

    pub fn fromJSON(&self, source: String) -> Superblock {
        let json_string = "{\"magic_number\":0x70736575646F4653,\"total_blocks\":\"temp\",\"free_blocks\":\"temp\",\"total_inodes\":temp,\"free_inodes\":temp}}";

        let superblock: Superblock = serde_json::from_str(&json_string).unwrap();
        return superblock;
    }
}
