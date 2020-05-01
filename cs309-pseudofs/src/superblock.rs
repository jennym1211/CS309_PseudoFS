use serde::{Deserialize, Serialize};
use crate::disk::block::Block;
use crate::inode::Inode;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Superblock {
    pub magic_number: String, // 0x70736575646F4653 magic number
    pub total_blocks: u32,
    pub free_blocks: Vec<Block>,
    pub inodes_vec: Vec<Inode>,
}

impl Superblock {

    pub fn new(magic_number: String, total_blocks: u32,free_blocks: Vec<Block>, inodes_vec: Vec<Inode> )-> Superblock
    {
        Superblock
        {
            magic_number: magic_number,
            total_blocks:total_blocks,
            free_blocks: free_blocks,
            inodes_vec: inodes_vec
        }
    }

    pub fn get_total_inodes(&self) -> usize
    {
        let mut inode_size = self.inodes_vec.len();

        return inode_size;

    }

    pub fn toJSON(&self) {
        let serialized_block = serde_json::to_string(&self).unwrap();

        println!("{}", serialized_block);
    }
    pub fn fromJSON(&self, source: String) -> Superblock {
       

        let superblock: Superblock = serde_json::from_str(&source).unwrap();
        return superblock;
    }
}
