use crate::directory::Directory;
use crate::disk::block::Block;
use crate::disk::Disk;
use crate::inode::Inode;
use crate::superblock::Superblock;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileSystem {
    pub disk: Disk,
    pub superblock: Superblock,
    pub directory: Directory,
    pub inodes: Vec<Inode>,
    pub inodes_per_block: u32,
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
            //diagnostics.push_str(&self.superblock.inodes_vec.to_string());
            diagnostics.push_str("\t \t Free Inodes: ");
            //diagnostics.push_str(&self.superblock.total_inodes.to_string());
            //diagnostics.push_str("\t \t Used Inodes: ");
            diagnostics.push_str("\n \t Blocks: ");
            diagnostics.push_str(&self.superblock.total_blocks.to_string());
            //diagnostics.push_str("\t \t Free Blocks: ");
            //diagnostics.push_str(&self.superblock.total_inodes.to_string());
            //diagnostics.push_str("\t \t Used Blocks: ");

            println!("{}", diagnostics);
        } else {
            println!("File System not mounted. No diagnostics available.");
        }
    }

    pub fn create(fileName: String) -> bool {
        let mut file = File::create(fileName);

        return false;
    }

    pub fn format(fileName: String) -> bool {
        let magic_number = String::from("0x70736575646F4653"); // 0x70736575646F4653 is always the magic number
        let mut total_blocks: u32 = 0;
        let mut total_inodes: u32 = 0;
        let mut free_blocks_vec: Vec<Block> = Vec::new();
        let mut inodes_vec: Vec<Inode> = Vec::new();
        let mut super_block =
            Superblock::new(magic_number, total_blocks, free_blocks_vec, inodes_vec);

        return true;
    }

    pub fn mount(&self, file_name: String) -> bool {
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
    pub fn readInode(inode_number: u32) {}

    pub fn writeInode(inode_num: u32, updated_inode: Inode) -> bool {
        return true;
    }

    pub fn getFreeInode() {}

    pub fn readBlock(blockID: u32) {}

    pub fn writeBlock(blockID: u32, block: Block) -> bool {
        return true;
    }
    pub fn getFreeBlock() {}

    pub fn toJSON(&self) {
        let serialized_block = serde_json::to_string(&self).unwrap();

        println!("{}", serialized_block);
    }

    pub fn fromJSON(source: String) -> FileSystem {
        let filesystem: FileSystem = serde_json::from_str(&source).unwrap();
        return filesystem;
    }
}
