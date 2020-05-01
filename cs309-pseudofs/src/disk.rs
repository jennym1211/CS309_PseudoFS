use block::Block;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Disk {
    pub disk_content: Vec<String>,
    pub blocks: u32,
    pub reads: u128,
    pub writes: u128,
    pub mounted: bool,
}

//Disk emulator functions
impl Disk {
    pub fn open(&self, file_name: String) -> bool {
        let path = Path::new(&file_name);
        //get line count
        return true;
    }

    pub fn close(disk: Disk) -> bool {
        return true;
    }

    pub fn read(&self, blockID: u32) -> Block {
        //need to increment
        self.reads = self.reads + 1;

        return Block.fromJSON(self.disk_content[blockID as usize]);
    }

    pub fn write(&self, blockID: u32, block: Block) -> bool {
        if block.get_blockid() >= &0
            && block.get_blockid() < &(self.disk_content.len() as u32)
            && *self.is_mounted() == true
        {
            self.writes = self.writes + 1;
            //self.disk_content[*block.get_blockid() as usize] = block.toJSON();
            return true;
        } else {
            return false;
        }
    }

    //Getters
    pub fn get_reads(&self) -> &u128 {
        return &self.reads;
    }

    pub fn get_writes(&self) -> &u128 {
        return &self.writes;
    }

    pub fn get_blocks(&self) -> &u32 {
        return &self.blocks;
    }

    pub fn is_mounted(&self) -> &bool {
        return &self.mounted;
    }
}

pub mod block {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Block {
        pub blockID: u32,
        pub nextNode: u32,
        pub data: String,
    }

    impl Block {
        pub fn new(blockID: u32, nextNode: u32, data: String) -> Block {
            Block {
                blockID: blockID,
                nextNode: nextNode,
                data: data,
            }
        }

        //Getters
        pub fn get_blockid(&self) -> &u32 {
            return &self.blockID;
        }

        pub fn get_next_node(&self) -> &u32 {
            return &self.nextNode;
        }

        pub fn get_data(&self) -> &String {
            return &self.data;
        }

        //Setters
        fn set_blockID(&mut self) -> &mut u32 {
            &mut self.blockID
        }

        fn set_nextNode(&mut self) -> &mut u32 {
            &mut self.nextNode
        }

        fn set_data(&mut self) -> &mut String {
            &mut self.data
        }

        /*
            Serialize disk to a JSON string
        */
        pub fn toJSON(&self) {
            let serialized_block = serde_json::to_string(&self).unwrap();

            println!("{}", serialized_block);
        }

        /*
            Return a block object from JSON string
        */
        pub fn fromJSON(source: String) -> Block {
            let block: Block = serde_json::from_str(&source).unwrap();
            return block;
        }
    }
}
