use crate::directory::Directory;
use crate::disk::block::Block;
use crate::disk::Disk;
use crate::inode::Inode;
use crate::inode::InodeType;
use crate::superblock::Superblock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

const INODES_PER_BLOCK: u32 = 50;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileSystem {
    pub disk: Disk,
    pub superblock: Superblock,
    pub root: Directory,
    pub inodes: Vec<Inode>,
    pub mounted: bool,
}

impl FileSystem {
    /**
     * Prints diagnostic and usage information about the mounted file system
     */
    pub fn diagnostics(&self) {
        if self.disk.mounted == true && self.mounted == true
        //if disk is mounted and filesystem is mounted
        {
            let mut used_inodes =
                self.superblock.get_total_inodes() - self.superblock.free_inode_count();
            let mut used_blocks =
                self.superblock.get_totalblocks() - self.superblock.free_block_count();
            let mut diagnostics = String::from("File system magic number is: ");

            if self.mounted == true {
                diagnostics.push_str("Valid \n");
            } else {
                diagnostics.push_str("Invalid \n");
            }

            diagnostics.push_str("\t Reads: ");
            diagnostics.push_str(&self.disk.reads.to_string());
            diagnostics.push_str("\t Writes: ");
            diagnostics.push_str(&self.disk.writes.to_string());
            diagnostics.push_str("\n \t Inodes: ");
            diagnostics.push_str(&self.superblock.get_total_inodes().to_string());
            diagnostics.push_str("\t \t Free Inodes: ");
            diagnostics.push_str(&self.superblock.free_inode_count().to_string());
            diagnostics.push_str("\t \t Used Inodes: ");
            diagnostics.push_str(&used_inodes.to_string());
            diagnostics.push_str("\n \t Blocks: ");
            diagnostics.push_str(&self.superblock.total_blocks.to_string());
            diagnostics.push_str("\t \t Free Blocks: ");
            diagnostics.push_str(&self.superblock.free_block_count().to_string());
            diagnostics.push_str("\t \t Used Blocks: ");
            diagnostics.push_str(&used_blocks.to_string());

            println!("{}", diagnostics);
        } else {
            println!("File System not mounted. No diagnostics available.");
        }
    }

    pub fn create(fileName: String) -> bool {
        let mut file = File::create(fileName);

        return false;
    }

    pub fn format(&mut self, fileName: String) -> bool {
        let magic_number = String::from("0x70736575646F4653"); // 0x70736575646F4653 is always the magic number
        let mut total_blocks: u32 = 0;
        let mut total_inodes: u32 = 0;
        let mut free_blocks_vec: Vec<Block> = Vec::new();
        let mut inodes_vec: Vec<Inode> = Vec::new();
        let mut super_block = Superblock::default();
        let inode_block = 1;

        let line_num = 7usize;

        let path = Path::new(&fileName);

        //Create the inodes
        for i in 0..self.inodes.len() {
            self.inodes
                .push(Inode::new(i as u32, InodeType::Free, -1, 0, Utc::now()));
        }

        return true;
    }

    pub fn mount(&mut self, file_name: String) -> bool {
        if *self.disk.is_mounted() == false && *self.is_mounted() == false {
            self.disk.open(file_name);
            self.superblock
                .fromJSON(self.disk.read(0).get_data().to_string());

            //self.inodes =

            let mut inode_block_num = 1;
            let mut current_node = 0;
        }

        return true;
    }

    pub fn unmount() -> bool {
        return true;
    }

    pub fn is_mounted(&self) -> &bool {
        return &self.mounted;
    }
    pub fn readInode(&self, inode_number: u32) -> Inode {
        if inode_number < 0 && inode_number > self.superblock.get_inodes().len() as u32 {
            panic!("Invalid inode number!");
        } else {
            return self.superblock.get_inodes()[inode_number as usize].clone();
        }
    }

    pub fn write_cache(&mut self) {
        let mut sb = Block::new(0, -1, self.superblock.toJSON());
        self.disk.write(sb);

        let mut inodeBlock = 1;
        let mut rootBlockID = self
            .readInode(self.root.get_inode_num("/".to_string()))
            .get_start_block();
        let mut start = 0;

        let mut end = INODES_PER_BLOCK;

        let mut temp: Vec<Inode> = Vec::new();
    }

    pub fn writeInode(inode_num: u32, updated_inode: Inode) -> bool {
        return true;
    }

    pub fn getFreeInode() {}

    pub fn readBlock(&mut self, blockID: u32) -> Block {
        let mut block = Block::default();

        if *self.disk.is_mounted() == true && *self.is_mounted() == true {
            return self.disk.read(blockID);
        } else {
            return block;
        }
    }

    pub fn writeBlock(&mut self, block: Block) -> bool {
        let mut result: bool = false;
        if *self.disk.is_mounted() == true && *self.is_mounted() == true {
            result = self.disk.write(block);
        }
        return result;
    }
    pub fn getFreeBlock() {}

    pub fn toJSON(&self) -> String {
        let serialized_block = serde_json::to_string(&self).unwrap();

        return String::from(serialized_block);
    }

    pub fn fromJSON(source: String) -> FileSystem {
        let filesystem: FileSystem = serde_json::from_str(&source).unwrap();
        return filesystem;
    }
}
