use block::Block;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Disk {
    pub diskName: String,
    pub disk_content: Vec<String>,
    pub blocks: i32,
    pub reads: u128,
    pub writes: u128,
    pub mounted: bool,
}

//Disk emulator functions
impl Disk {
    pub fn default() -> Disk {
        let mut disk_content: Vec<String> = Vec::new();
        let mut diskName = "";
        Disk {
            diskName: "".to_string(),
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
    pub fn open(&mut self, file_name: String) -> std::io::Result<()> {
        if *self.is_mounted() == false {
            println!("Disk image loading...Reading files...");
            let mut path = Path::new(&file_name);
            let line_num = 7usize;
            //let line = self.get_line_at(&path, line_num - 1);
            //println!("{}", line.unwrap());

            //self.disk_content = Vec::with_capacity(line);

            //let mut file = File::create(file_name.clone())?;
            let mut f = File::open(file_name.clone())?;
            let mut br = BufReader::new(f);

            for i in br.lines() {
                self.disk_content.push(i?);
            }
            self.mounted = true;

            println!("Disk image loaded and ready to run...");
            Ok(())
        } else {
            panic!("Disk image unmounted...Please mount image file...");
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

            let mut payload = self.disk_content.clone();
            let mut f = File::create("./disks/tmp/foo").expect("Unable to create file");
            //f.write_all(payload.into_bytes()).expect("Unable to write payload");

            println!("Unmounting disk image...");
            self.mounted = false;
            return true;
        } else {
            eprintln!("Disk image already closed and unmounted...Please mount disk image...");
            return false;
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut file = File::open(self.diskName.clone())?;

        for i in 0..self.disk_content.len() {
            self.writes = self.writes + 1;
            file.write(self.disk_content[i].clone().as_bytes())?;
            file.write("\n".as_bytes());
        }
        file.flush();
        Ok(())
    }

    pub fn read(&mut self, blockID: i32) -> Block {
        self.reads = self.reads + 1;
        let mut block: Block = Block::from_json(self.disk_content[blockID as usize].to_string());
        return block;
    }

    pub fn read_superblock(&mut self) -> Block {
        //self.reads = self.reads + 1;
        let data_to_deserialize_ = r#"{"blockID":0,"nextNode":-1,"payload":"{\"magicNumber\":12345,\"blockCount\":1000,\"inodeCount\":100}"#;
        let block: Block = serde_json::from_str(&data_to_deserialize_).unwrap();
        return block;
    }

    pub fn write(&mut self, mut block: Block) -> bool {
        if block.get_blockid() >= &0
            && block.get_blockid() < &(self.disk_content.len() as i32)
            && *self.is_mounted() == true
        {
            self.writes = self.writes + 1;
            //self.disk_content[*block.get_blockid() as usize] = block.to_json();
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
        pub payload: String,
    }

    impl Block {
        pub fn default() -> Block {
            Block {
                blockID: 0,
                nextNode: 0,
                payload: String::from(""),
            }
        }
        pub fn new(blockID: i32, nextNode: i32, payload: String) -> Block {
            Block {
                blockID: blockID,
                nextNode: nextNode,
                payload: payload,
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
            return &self.payload;
        }

        //Setters
        pub fn set_blockID(&mut self, blockID: i32) -> &mut i32 {
            &mut self.blockID
        }

        pub fn set_nextNode(&mut self, nextNode: i32) -> &mut i32 {
            &mut self.nextNode
        }

        pub fn set_data(&mut self, payload: String) -> &mut String {
            &mut self.payload
        }

        /*
            Serialize disk to a JSON string
        */
        pub fn to_json(&mut self) -> String {
            let serialized_block = serde_json::to_string(&self).unwrap();
            return String::from(serialized_block);
        }

        /*
            Return a block object from JSON string
        */
        pub fn from_json(source: String) -> Block {
            let block: Block = serde_json::from_str(&source).unwrap();
            return block;
        }

        pub fn read_superblock(&mut self) -> Block {
            //self.reads = self.reads + 1;
            let data_to_deserialize_ = r#"{"blockID":0,"nextNode":-1,"payload":"{\"magicNumber\":12345,\"blockCount\":1000,\"inodeCount\":100}"#;
            let block: Block = serde_json::from_str(&data_to_deserialize_).unwrap();
            return block;
        }
    }
}
