use crate::directory::Directory;
use crate::disk::block::Block;
use crate::disk::Disk;
use crate::inode::{Inode, InodeType, Inodes};
use crate::superblock::Superblock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;
use std::path::Path;

const INODES_PER_BLOCK: i32 = 50;
const NEG_ONE: i32 = -1;
const NEG_TWO: i32 = -2;

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

    pub fn create(fileName: String, sizeInKB: usize) -> std::io::Result<()> {
        let mut file = File::create(fileName)?;
        let mut lw = LineWriter::with_capacity(sizeInKB, file);
        lw.write(b"\r\n");
        Ok(())
    }

    pub fn format(&mut self, fileName: String) -> bool {
        let magic_number = String::from("0x70736575646F4653"); // 0x70736575646F4653 is always the magic number
        let mut total_blocks: usize = 0;
        let mut total_inodes: usize = 0;
        let mut super_block = Superblock::default();
        let inode_block = 1;

        let line_num = 7usize;

        let path = Path::new(&fileName);
        total_blocks = path.iter().count();
        total_inodes = total_blocks / 10;

        self.inodes = Vec::with_capacity(total_blocks);

        self.disk.open(fileName);

        //Create the inodes
        for i in 0..self.inodes.len() {
            self.inodes
                .push(Inode::new(i as i32, InodeType::Free, -1, 0, Utc::now()));
        }
        let mut rootBlockID = (total_inodes / INODES_PER_BLOCK as usize) + 1;

        self.inodes[0].set_inodetype(InodeType::Directory);
        self.inodes[0].set_startblock(rootBlockID as i32);
        self.inodes[0].set_size(0);

        super_block.set_magicNumber(magic_number);
        super_block.set_blockCount(total_blocks as i32);
        super_block.setInodes(self.inodes.clone());

        self.disk.write(Block::new(0, -1, super_block.toJSON()));

        let mut start = 0;
        let mut end = INODES_PER_BLOCK;

        while inode_block != rootBlockID {
            let mut inode_block_increment = inode_block + 1;

            let mut temp: Vec<Inode> = Vec::new();

            temp.clone_from_slice(&self.inodes[start..end as usize]);
            let mut inode_collection: Inodes = Inodes::new_of_vec(temp);

            let mut iblock: Block = Block::new(
                inode_block_increment as i32,
                inode_block as i32,
                inode_collection.clone().toJSON(),
            );

            if inode_block == rootBlockID {
                iblock.set_nextNode(-1);
            }
            self.disk.write(iblock);
            start = end as usize;
            end = end + INODES_PER_BLOCK;
            if end > self.inodes.len().clone() as i32 {
                end = self.inodes.len() as i32;
            }
        }

        let mut rootBlock = Block::new(rootBlockID as i32, -1, self.root.toJSON());
        self.disk.write(rootBlock);

        for i in 0..self.superblock.get_totalblocks() {
            let i: i32 = rootBlockID as i32;
            let mut blank: Block = Block::new(i, -2, "".to_string());
            self.disk.write(blank);
        }

        return self.disk.close();
    }

    pub fn mount(&mut self, file_name: String) -> bool {
        if *self.disk.is_mounted() == false && *self.is_mounted() == false {
            self.disk.open(file_name);
            self.superblock
                .fromJSON(self.disk.read(0).get_data().to_string());

            let mut div_10 = self.superblock.get_totalblocks() / 10;

            self.inodes = Vec::with_capacity(div_10);
            let mut inode_block_num = 1;
            let mut current_inode = 0;

            while inode_block_num != -1 {
                let inodeBlock: Block = self.disk.read(inode_block_num);
                inode_block_num = inodeBlock.get_next_node().clone();
                let inode_collection: Inodes = Inodes::fromJSON(inodeBlock.get_data().to_string());

                let mut temp: Vec<Inode> = inode_collection.get_inodes();

                for i in 0..inode_collection.get_inodes().len() {
                    self.inodes[current_inode] = temp[i].clone();
                    current_inode = current_inode + 1;
                }
            }

            let mut rootBlock = self.inodes[0].get_start_block();
            self.root =
                Directory::fromJSON(self.disk.read(rootBlock.clone()).get_data().to_string());

            self.superblock.setInodes(self.inodes.clone());

            let i = 2;

            for i in 0..self.superblock.get_totalblocks() {
                let blk: Block = self.disk.read(i as i32);
                if blk.get_next_node().clone() == NEG_TWO {
                    self.superblock.put_free_block(blk);
                }
            }
            self.mounted = true;
        }

        return self.mounted;
    }

    pub fn sync(&mut self) {
        if *self.disk.is_mounted() == true && *self.is_mounted() == true {
            self.write_cache();
            self.disk.run();
        } else {
            eprintln!("Cannot sync.");
        }
    }

    pub fn unmount(&mut self) -> bool {
        if *self.disk.is_mounted() == true && *self.is_mounted() == true {
            self.write_cache();
            self.disk.close();

            self.superblock = Superblock::default();
            self.disk = Disk::default();
            self.mounted = false;
        }
        return true;
    }

    pub fn getFileNameInode(&mut self, name: String) -> Inode {
        let mut node = Inode::default();

        if self.root.get_inode_num(name.clone()) >= 0 {
            node = self.readInode(self.root.get_inode_num(name));
        }
        return node;
    }

    pub fn readFile(&mut self, name: String) -> String {
        let mut processing: bool = true;
        let mut contents = String::from("");
        let mut inode: Inode = self.readInode(self.root.get_inode_num(name));
        let mut block: Block = self.readBlock(inode.get_start_block().clone()).clone();

        while processing {
            contents.push_str(&block.get_data().to_string());

            if block.get_next_node() == &NEG_ONE {
                processing = false
            } else {
                block = self.readBlock(*block.get_next_node());
            }
        }

        return contents;
    }

    pub fn writeFile(&mut self, name: String, contents: String) -> bool {
        let mut blocks: Vec<Block> = Vec::new();
        let mut start = 0;
        let mut end = 0;

        let mut inode: Inode = self.getFileNameInode(name.clone()).clone();

        if inode == Inode::default() {
            inode = self.superblock.get_free_inode();
            inode.set_cTime(Utc::now());
            inode.set_inodetype(InodeType::File);
            self.root.add(inode.get_inodenum().clone(), name);
        } else {
            let mut processing: bool = true;
            let mut block: Block = self.readBlock(inode.get_start_block().clone());

            while processing {
                if block.get_next_node().clone() == NEG_ONE {
                    processing = false;
                }
                block.set_nextNode(-2);
                block.set_data("".to_string());
                self.writeBlock(block.clone());
                self.superblock.put_free_block(block.clone());
            }
        }
        while start < contents.len() {
            end = end + 1000;

            if end > contents.len() {
                end = contents.len();
            }
            let mut block = self.superblock.get_free_block();
            if block.clone() == Block::default() {
                for block_item in blocks.iter_mut() {
                    block_item.set_nextNode(-2);
                    block_item.set_data("".to_string());
                    self.superblock.put_free_block(block_item.clone());
                }
                return false;
            }

            let mut substring = &contents[start..end];
            block.set_data(substring.to_string());
            blocks.push(block);
            start = end;
        }

        inode.set_startblock(blocks[0].get_blockid().clone());
        inode.set_size(blocks.len() as i32);
        self.superblock.update_inode(&inode);

        for i in 0..blocks.len() {
            let mut blockindex = blocks[i + 1].clone();
            if i < (blocks.len() - 1) {
                blocks[i].set_nextNode(blockindex.blockID);
            } else {
                blocks[i].set_nextNode(-1);
            }
        }
        for block_item in blocks.iter() {
            self.writeBlock(block_item.clone());
        }
        self.sync();
        return true;
    }

    pub fn deleteFile(&mut self, name: String) -> bool {
        let mut processing: bool = true;
        let mut inode =
            self.superblock.get_inodes()[self.root.get_inode_num(name.clone()) as usize].clone();
        if inode.get_inodenum().clone() == self.root.get_inode_num("/".to_string()) {
            eprintln!("Cannot delete root directory!");
            return false;
        }
        let mut block: Block = self.readBlock(inode.get_start_block().clone()).clone();
        while processing {
            let mut nextNode = block.get_next_node().clone();
            block.set_nextNode(-2);
            block.set_data("".to_string());
            self.writeBlock(block.clone());

            if nextNode.clone() == NEG_ONE {
                processing = false;
            } else {
                block = self.readBlock(nextNode.clone());
            }
        }
        inode.set_size(0);
        inode.set_startblock(-1);
        inode.set_inodetype(InodeType::Free);
        self.superblock.update_inode(&inode);
        self.root.remove(name);
        self.sync();
        return true;
    }

    pub fn list(&mut self) -> Vec<String> {
        let mut fileNames: Vec<String> = Vec::new();

        fileNames.append(&mut self.root.getContents());

        let mut list: Vec<String> = Vec::with_capacity(fileNames.len().clone());
        //ToDO: sort
        for i in 0..fileNames.len() {
            list[i] = fileNames[i].clone();
        }

        return list;
    }

    pub fn is_mounted(&self) -> &bool {
        return &self.mounted;
    }
    pub fn readInode(&self, inode_number: i32) -> Inode {
        if inode_number < 0 && inode_number > self.superblock.get_inodes().len() as i32 {
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

    pub fn writeInode(&mut self, inode_num: i32, updated_inode: Inode) -> bool {
        return true;
    }

    pub fn readBlock(&mut self, blockID: i32) -> Block {
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

    pub fn toJSON(&self) -> String {
        let serialized_block = serde_json::to_string(&self).unwrap();

        return String::from(serialized_block);
    }

    pub fn fromJSON(source: String) -> FileSystem {
        let filesystem: FileSystem = serde_json::from_str(&source).unwrap();
        return filesystem;
    }
}
