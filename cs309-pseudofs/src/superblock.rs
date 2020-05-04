use crate::disk::block::Block;
use crate::inode::Inode;
use crate::inode::InodeType;
use serde::{Deserialize, Serialize};

const VALID_MAGIC_NUM: &str = "0x70736575646F4653"; //always the magic number for pseudo FS

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Superblock {
    pub magic_number: String, // 0x70736575646F4653 magic number
    pub total_blocks: i32,
    pub total_inodes: i32,
    pub free_blocks: Vec<Block>,
    pub free_inodes: Vec<Inode>,
    pub inodes_vec: Vec<Inode>,
}

impl Superblock {
    pub fn default() -> Superblock {
        let mut inodes_vec: Vec<Inode> = Vec::new();
        let mut free_blocks: Vec<Block> = Vec::new();
        let mut free_inodes: Vec<Inode> = Vec::new();
        let mut total_inodes = 0;
        let mut total_blocks = 0;

        Superblock {
            magic_number: VALID_MAGIC_NUM.to_string(),
            total_blocks: total_blocks,
            free_blocks: free_blocks,
            inodes_vec: inodes_vec,
            free_inodes: free_inodes,
            total_inodes: total_inodes,
        }
    }

    pub fn new(
        &mut self,
        mut magic_number: String,
        mut total_blocks: i32,
        mut free_blocks: Vec<Block>,
        mut inodes_vec: Vec<Inode>,
    ) -> Superblock {
        let mut inode_vec_size = inodes_vec.len() as i32;
        let mut free_inodes: Vec<Inode> = Vec::new();

        self.setInodes(self.inodes_vec.clone());

        if self.magic_number == VALID_MAGIC_NUM.to_string() {
            magic_number = VALID_MAGIC_NUM.to_string();
        } else {
            eprintln!("Invalid magic number!");
            self.magic_number = VALID_MAGIC_NUM.to_string();
        }
        Superblock {
            magic_number: magic_number,
            total_blocks: total_blocks,
            free_blocks: free_blocks,
            free_inodes: free_inodes,
            inodes_vec: inodes_vec,
            total_inodes: inode_vec_size,
        }
    }

    pub fn set_magicNumber(&mut self, magic_number: String) -> &mut String {
        &mut self.magic_number
    }

    pub fn set_blockCount(&mut self, block_count: i32) -> &mut i32 {
        &mut self.total_blocks
    }

    pub fn setInodes(&mut self, newInodes: Vec<Inode>) {
        self.inodes_vec = Vec::with_capacity(newInodes.len());
        self.free_inodes.clear();

        for i in 0..self.inodes_vec.len() {
            self.inodes_vec.push(newInodes[i].clone());
            if self.inodes_vec[i].inode_type == InodeType::Free {
                self.put_free_inode(self.inodes_vec[i].clone());
            }
        }
    }

    pub fn update_inode(&mut self, inode: &Inode) -> bool {
        if inode.clone().get_inodenum() >= &0 && inode.clone().get_inodenum() < &self.total_inodes {
            self.inodes_vec[*inode.clone().get_inodenum() as usize] = inode.clone();

            return true;
        } else {
            return false;
        }
    }

    pub fn free_inode_count(&self) -> usize {
        return self.free_inodes.len();
    }

    pub fn free_block_count(&self) -> usize {
        return self.free_blocks.len();
    }

    pub fn put_free_inode(&mut self, inode: Inode) {
        self.free_inodes.push(inode);
    }

    pub fn get_free_inode(&self) -> Inode {
        if self.free_inodes.len() < 0 {
            panic!("No more free inodes!");
        } else {
            return self.free_inodes.clone().remove(0);
        }
    }

    pub fn put_free_block(&mut self, block: Block) {
        self.free_blocks.push(block);
    }

    pub fn get_free_block(&self) -> Block {
        if self.free_blocks.len() < 0 {
            panic!("No more free blocks!");
        } else {
            return self.free_blocks.clone().remove(0);
        }
    }

    pub fn get_inodes(&self) -> Vec<Inode> {
        let mut inode_copy: Vec<Inode> = Vec::with_capacity(self.inodes_vec.len());
        let mut inodes_vec = self.inodes_vec.clone();
        for i in 0..self.inodes_vec.len().clone() {
            inode_copy.push(inodes_vec[i].clone());
        }
        return inode_copy.clone();
    }

    pub fn get_total_inodes(&self) -> usize {
        let mut inode_size = self.inodes_vec.len();

        return inode_size;
    }

    pub fn get_totalblocks(&self) -> usize {
        return self.total_blocks as usize;
    }

    pub fn to_json(&self) -> String {
        let serialized_block = serde_json::to_string(&self).unwrap();
        return String::from(serialized_block);
    }
    pub fn from_json(source: String) -> Superblock {
        let superblock: Superblock = serde_json::from_str(&source).unwrap();
        return superblock;
    }
}
