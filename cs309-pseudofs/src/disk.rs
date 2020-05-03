use block::Block;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Disk {
    pub disk_content: Vec<String>,
    pub blocks: i32,
    pub reads: u128,
    pub writes: u128,
    pub mounted: bool,
}

//Disk emulator functions
impl Disk {
    pub fn default() -> Disk {
        let disk_content: Vec<String> = Vec::new();
        Disk {
            disk_content: disk_content,
            blocks: 0,
            mounted: false,
            reads: 0,
            writes: 0,
        }
    }

    /*
        Got help from link below with opening and counting lines in a file:
        https://www.rosettacode.org/wiki/Read_a_specific_line_from_a_file#Rust
    */
    pub fn open(&self, file_name: String) -> bool {
        if *self.is_mounted() == false {
            println!("Disk image loading...Reading files...");
            let path = Path::new(&file_name);
            let line_num = 7usize;
            let line = self.get_line_at(&path, line_num - 1);
            println!("{}", line.unwrap());
            println!("Disk image loaded and ready to run...");
            return true;
        } else {
            eprintln!("Disk image unmounted...Please mount image file...");
            return false;
        }
    }

    pub fn get_line_at(&self, path: &Path, line_num: usize) -> Result<String, Error> {
        let file = File::open(path).expect("File not found or cannot be opened");
        let content = BufReader::new(&file);
        let mut lines = content.lines();
        lines.nth(line_num).expect("No line found at that position")
    }

    pub fn close(&mut self) -> bool {
        if *self.is_mounted() == true {
            println!("Finishing writing jobs and closing disk image...");

            let mut data = self.disk_content.clone();
            let mut f = File::create("/tmp/foo").expect("Unable to create file");
            //f.write_all(data.into_bytes()).expect("Unable to write data");

            println!("Unmounting disk image...");
            self.mounted = false;
            return true;
        } else {
            eprintln!("Disk image already closed and unmounted...Please mount disk image...");
            return false;
        }
    }

    pub fn run(&self) {}

    pub fn read(&mut self, blockID: i32) -> Block {
        self.reads = self.reads + 1;
        let mut block = Block::default();
        block.fromJSON(self.disk_content[blockID as usize].to_string());
        return block;
    }

    pub fn write(&mut self, mut block: Block) -> bool {
        if block.get_blockid() >= &0
            && block.get_blockid() < &(self.disk_content.len() as i32)
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

    pub fn get_blocks(&self) -> &i32 {
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
        pub blockID: i32,
        pub nextNode: i32,
        pub data: String,
    }

    impl Block {
        pub fn default() -> Block {
            Block {
                blockID: 0,
                nextNode: 0,
                data: String::from(""),
            }
        }
        pub fn new(blockID: i32, nextNode: i32, data: String) -> Block {
            Block {
                blockID: blockID,
                nextNode: nextNode,
                data: data,
            }
        }

        //Getters
        pub fn get_blockid(&self) -> &i32 {
            return &self.blockID;
        }

        pub fn get_next_node(&self) -> &i32 {
            return &self.nextNode;
        }

        pub fn get_data(&self) -> &String {
            return &self.data;
        }

        //Setters
        pub fn set_blockID(&mut self, blockID: i32) -> &mut i32 {
            &mut self.blockID
        }

        pub fn set_nextNode(&mut self, nextNode: i32) -> &mut i32 {
            &mut self.nextNode
        }

        pub fn set_data(&mut self, data: String) -> &mut String {
            &mut self.data
        }

        /*
            Serialize disk to a JSON string
        */
        pub fn toJSON(&mut self) -> String {
            let serialized_block = serde_json::to_string(&self).unwrap();
            return String::from(serialized_block);
        }

        /*
            Return a block object from JSON string
        */
        pub fn fromJSON(&mut self, source: String) -> Block {
            let block: Block = serde_json::from_str(&source).unwrap();
            return block;
        }
    }
}
