use crate::directory::Directory;
use crate::disk::block::Block;
use crate::disk::Disk;
use crate::inode::{Inode, InodeType, Inodes};
use crate::superblock::Superblock;
use std::fs::OpenOptions;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::io::LineWriter;
use std::io::{BufWriter, Write};
use std::iter;
use std::path::Path;
use std::path::PathBuf;

const INODES_PER_BLOCK: i32 = 50;
const NEG_ONE: i32 = -1;
const NEG_TWO: i32 = -2;
const VALID_MAGIC_NUM: i64 = 0x70736575646F4653; //always the magic number for pseudo FS

/**
 *  A data structure that represents the over PseudoFS.
 */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileSystem {
    pub disk: Disk,
    pub superblock: Superblock,
    pub root: Directory,
    pub inodes: Vec<Inode>,
    pub mounted: bool,
}

impl FileSystem {
    pub fn default() -> FileSystem {
        let mut disk = Disk::default();
        let mut sb = Superblock::default();
        let mut root = Directory::new();
        let mut inodes: Vec<Inode> = Vec::new();
        let mut mounted = false;

        FileSystem {
            disk: disk,
            superblock: sb,
            root: root,
            inodes: inodes,
            mounted: mounted,
        }
    }
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

    pub fn create_disk(&mut self, mut fileName: String, sizeInKB: usize) -> bool {
        // let mut path = PathBuf::new();
        //path.join(fileName.clone());
        let mut file = File::create(fileName).expect("Could not create disk.");

        //self.disk.diskName = path.into_os_string().into_string().unwrap();

        //let mut lw = LineWriter::with_capacity(sizeInKB, file);
        //lw.write(b"\r\n").expect("Could not write to file.");
        //let mut bw = BufWriter::new(file);

        let mut sb: Superblock = Superblock::default();

        let mut sb_to_json: String = sb.to_json();

        file.write(sb_to_json.as_bytes())
            .expect("Could not write to file");

        for i in 0..sizeInKB {
            let newline = "\r\n";
            file.write(newline.as_bytes())
                .expect("Could not write to file.");
        }
        return true;
    }

    pub fn format(&mut self, fileName: String) -> bool {
        let magic_number: i64 = 0x70736575646F4653; // 0x70736575646F4653 is always the magic number
        let mut total_blocks: usize = 0;
        let mut total_inodes: usize = 0;
        let mut super_block = Superblock::default();
        let inode_block: i32 = 1;

        total_blocks = 1000;
        total_inodes = total_blocks / 10;

        self.inodes = Vec::with_capacity(total_blocks);

        self.disk
            .open(fileName.clone())
            .expect("Could not open disk.");

        //Create the inodes
        for i in 0..1000 {
            self.inodes = vec![Inode::new(i as i32, InodeType::Free, -1, 0, Utc::now()); 1000];
        }

        let mut rootBlockID = (total_inodes / INODES_PER_BLOCK as usize) + 1;

        // print!("Inodes: {:?}", self.inodes);

        self.inodes[0].set_inodetype(InodeType::Directory);
        self.inodes[0].set_startblock(rootBlockID as i32);
        self.inodes[0].set_size(0);

        //super_block.set_magic_number(magic_number);
        //super_block.set_block_count(total_blocks as i32);
        //super_block.set_inodes(self.inodes.clone());

        self.disk.write(Block::new(0, -1, super_block.to_json()));

        let mut start = 0;
        let mut end = INODES_PER_BLOCK;

        let mut inode_block_increment = inode_block + 1;
        let mut temp: Vec<Inode> = Vec::new();

        //temp.clone_from_slice(&self.inodes[start..end as usize]);
        temp = self.inodes[start..end as usize].iter().cloned().collect();
        let mut inode_collection: Inodes = Inodes::new_of_vec(temp);

        let mut iblock: Block = Block::new(
            inode_block_increment as i32,
            inode_block as i32,
            inode_collection.clone().to_json(),
        );

        if inode_block == rootBlockID as i32 {
            iblock.set_nextNode(-1);
        }
        self.disk.write(iblock);
        start = end as usize;
        end += INODES_PER_BLOCK;

        if end > self.inodes.len().clone() as i32 {
            end = self.inodes.len() as i32;
        }

        let mut rootBlock = Block::new(rootBlockID as i32, -1, self.root.to_json());
        self.disk.write(rootBlock);

        for rootBlockID in (0..self.superblock.get_totalblocks()).step_by(1) {
            let mut i = rootBlockID;

            let mut blank: Block = Block::new(i as i32, -2, "".to_string());

            self.disk.write(blank);
        }

        match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(fileName.to_string())
        {
            Ok(ref mut file) => {
                write!(file, "{}", self.superblock.to_json().to_string()).unwrap();
                write!(file, "{}", self.root.to_json().to_string()).unwrap();
                write!(file, "{}", self.to_json().to_string()).unwrap();
            }
            Err(err) => {
                panic!("Failed to open file: {}", err);
            }
        }

        return self.disk.close();
    }

    pub fn mount(&mut self, file_name: String) -> bool {
        if *self.disk.is_mounted() == false && *self.is_mounted() == false {
            self.disk
                .open(file_name)
                .expect("Could not open disk image.");

            let mut total_blocks: i32 = 100;
            let mut total_inodes: i32 = 1000;
            let magic_num = VALID_MAGIC_NUM;
            let mut free_inodes: Vec<Inode> = Vec::new();
            let mut free_blocks: Vec<Block> = Vec::new();
            let mut inodes_vec: Vec<Inode> = Vec::new();

            for i in 0..500 {
                free_inodes = vec![Inode::new(i as i32, InodeType::Free, -1, 0, Utc::now()); 500];
            }

            for i in 0..1000 {
                inodes_vec = vec![Inode::new(i as i32, InodeType::Free, -1, 0, Utc::now()); 1000];
            }

            for i in 0..50 {
                free_blocks = vec![Block::new(i as i32, i as i32, "".to_string()); 50];
            }

            self.superblock = Superblock::default();
            self.superblock.set_block_count(total_blocks);
            self.superblock.set_magic_number(magic_num);

            let mut div_10 = self.superblock.get_totalblocks() / 10;

            self.inodes = inodes_vec.clone();
            let mut inode_block_num = 1;
            let mut current_inode = 0;

            let mut rootBlock = self.inodes[0].get_start_block();
            self.root = Directory::new();

            self.superblock.set_inodes(self.inodes.clone());

            let i = 2;

            self.superblock.inodes_vec = inodes_vec.clone();
            self.superblock.total_blocks = total_blocks;
            self.superblock.total_inodes = total_inodes;
            self.superblock.free_inodes = free_inodes;
            self.superblock.free_blocks = free_blocks;
        }
        self.mounted = true;
        return self.mounted;
    }

    pub fn unmount(&mut self) -> bool {
        if *self.disk.is_mounted() == true && *self.is_mounted() == true {
            //self.write_cache();
            self.disk.close();

            self.superblock = Superblock::default();
            self.disk = Disk::default();
            self.mounted = false;
        }
        return true;
    }

    pub fn read_file(&self, filename: String) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors. Show the line and its number.
            println!("{}. {}", index + 1, line);
        }
    }

    /**
     *  
     *  Takes a block of data, and writes it to the disk.
     *
     */
    pub fn copy_in(&mut self, fileName: String) -> std::io::Result<()> {
        println!("Copying data into disk...");

        let mut testBlock: Block = Block::new(
            2,
            NEG_TWO,
            "This is a test for the copy in function.".to_string(),
        );

        match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(fileName.to_string())
        {
            Ok(ref mut file) => {
                writeln!(file, "{}", testBlock.to_json().to_string()).unwrap();
            }
            Err(err) => {
                panic!("Failed to open file: {}", err);
            }
        }
        println!("Copy complete!");
        self.disk.writes = self.disk.writes + 1;
        Ok(())
    }

    pub fn copy_out(&self) -> std::io::Result<()> {
        println!("Starting copy of file on directory...");
        //let mut end = &self.get_next_node(&self);
        //let mut start = //get the location of file to copy
        let mut dest_path_name = "./disk.txt";
        let mut start_path_name = "./disks/disk4.disk";
        fs::copy(start_path_name, dest_path_name)?;
        println!("Copy complete!");
        Ok(())
    }

    pub fn delete_file(&mut self, name: String) -> bool {
        let mut processing: bool = true;
        let mut inode =
            self.superblock.get_inodes()[self.root.get_inode_num(name.clone()) as usize].clone();
        if inode.get_inodenum().clone() == self.root.get_inode_num("/".to_string()) {
            eprintln!("Cannot delete root directory!");
            return false;
        }
        let mut block: Block = self.read_block(inode.get_start_block().clone()).clone();
        while processing {
            let mut next_node = block.get_next_node().clone();
            block.set_nextNode(-2);
            block.set_data("".to_string());
            self.write_block(block.clone());

            if next_node.clone() == NEG_ONE {
                processing = false;
            } else {
                block = self.read_block(next_node.clone());
            }
        }
        inode.set_size(0);
        inode.set_startblock(-1);
        inode.set_inodetype(InodeType::Free);
        self.superblock.update_inode(&inode);
        self.root.remove(name);
        return true;
    }

    pub fn is_mounted(&self) -> &bool {
        return &self.mounted;
    }

    pub fn write_inode(&mut self, inode_num: i32, updated_inode: Inode) -> bool {
        return true;
    }

    pub fn read_block(&mut self, block_id: i32) -> Block {
        let mut block = Block::default();

        if *self.disk.is_mounted() == true && *self.is_mounted() == true {
            return self.disk.read(block_id);
        } else {
            return block;
        }
    }

    pub fn write_block(&mut self, block: Block) -> bool {
        let mut result: bool = false;
        if *self.disk.is_mounted() == true && *self.is_mounted() == true {
            result = self.disk.write(block);
        }
        return result;
    }

    pub fn to_json(&self) -> String {
        let serialized_block = serde_json::to_string_pretty(&self).unwrap();

        return String::from(serialized_block);
    }

    pub fn from_json(source: String) -> FileSystem {
        let filesystem: FileSystem = serde_json::from_str(&source).unwrap();
        println!("{:?}", filesystem);
        return filesystem;
    }
}
