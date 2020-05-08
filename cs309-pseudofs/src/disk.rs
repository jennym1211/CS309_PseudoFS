use block::Block;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

/**
 * The struct that represents a disk image.
 *
 */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Disk {
    pub disk_name: String,
    pub disk_content: Vec<String>,
    pub blocks: i32,
    pub reads: u128,
    pub writes: u128,
    pub mounted: bool,
}

//Disk emulator functions
impl Disk {
    //Default object creation for a disk.
    pub fn default() -> Disk {
        let mut disk_content: Vec<String> = Vec::new();
        let mut disk_name = "";
        Disk {
            disk_name: "".to_string(),
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

    /**
     * Write any changes out to the disk and close the file, and flag the disk as unmounted
     * return true if successful, false otherwise
     */
    pub fn close(&mut self) -> bool {
        if *self.is_mounted() == true {
            println!("Finishing writing jobs and closing disk image...");

            let mut payload = self.disk_content.clone();
            //let mut f = File::create("./disks/tmp/foo").expect("Unable to create file");
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
        let mut file = File::open(self.disk_name.clone())?;

        for i in 0..self.disk_content.len() {
            self.writes = self.writes + 1;
            file.write(self.disk_content[i].clone().as_bytes())?;
            file.write("\n".as_bytes());
        }
        file.flush().expect("Could not flush file.");
        Ok(())
    }

    /**
     * Read a block of data from the disk
     * return The block stored on this line, or null if the block ID is invalid
     */
    pub fn read(&mut self, block_id: i32) -> Block {
        self.reads = self.reads + 1;
        let mut block: Block = Block::from_json(self.disk_content[block_id as usize].to_string());
        return block;
    }

    /**
     * Write a block of data to the disk
     * return true if the write was successful, false otherwise
     */
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

    /**
     * Returns the total number of reads this disk has had since being mounted
     */
    pub fn get_reads(&self) -> &u128 {
        return &self.reads;
    }

    /**
     *  Return the total number of writes this disk has had since being mounted
     */
    pub fn get_writes(&self) -> &u128 {
        return &self.writes;
    }

    /**
     * Gets blocks.
     */
    pub fn get_blocks(&self) -> &i32 {
        return &self.blocks;
    }

    /**
     * Returns if mounted or not.
     */
    pub fn is_mounted(&self) -> &bool {
        return &self.mounted;
    }
}

pub mod block {
    use serde::{Deserialize, Serialize};

    /**
     * Struct that reprents a block of data.
     */
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Block {
        pub block_id: i32,
        pub next_node: i32,
        pub payload: String,
    }

    impl Block {
        /**
         * Returns the default new block object.
         */
        pub fn default() -> Block {
            Block {
                block_id: 0,
                next_node: 0,
                payload: String::from(""),
            }
        }

        /**
         * Returns a new block object with specified parameters.
         */
        pub fn new(block_id: i32, next_node: i32, payload: String) -> Block {
            Block {
                block_id: block_id,
                next_node: next_node,
                payload: payload,
            }
        }

        //Getters
        /**
         * Gets the block ID.
         */
        pub fn get_blockid(&self) -> &i32 {
            return &self.block_id;
        }

        /*
            Gets number of next node.
        */
        pub fn get_next_node(&self) -> &i32 {
            return &self.next_node;
        }

        /*
            Gets the data off the block.
        */
        pub fn get_data(&self) -> &String {
            return &self.payload;
        }

        //Setters
        pub fn set_blockID(&mut self, block_id: i32) -> &mut i32 {
            &mut self.block_id
        }

        pub fn set_nextNode(&mut self, next_node: i32) -> &mut i32 {
            &mut self.next_node
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
            println!("{:?}", block);
            return block;
        }
    }
}
