use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/**
 * Data structure that reprents an inode.
 */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Inode {
    pub inode_num: i32,
    pub inode_type: InodeType,
    pub start_block: i32,
    pub size: i32,
    pub c_time: DateTime<Utc>, //find a date/time datastruct idk
}

/**
 *  Vec of inodes data structure.
 */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Inodes {
    pub inodes: Vec<Inode>,
}

/**
 *  Enums for inode types that exist on a file system.
 */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum InodeType {
    Free,
    File,
    Directory,
    Symlink,
}

impl Inode {
    //Default object creation for a inode
    pub fn default() -> Inode {
        let mut c_time: DateTime<Utc> = Utc::now();
        Inode {
            inode_num: 0,
            inode_type: InodeType::File,
            start_block: 0,
            size: 0,
            c_time: c_time,
        }
    }

    /**
     * Returns a new inode object with specified parameters.
     */
    pub fn new(
        mut inode_num: i32,
        mut inode_type: InodeType,
        mut start_block: i32,
        mut size: i32,
        mut c_time: DateTime<Utc>,
    ) -> Inode {
        Inode {
            inode_num: inode_num,
            inode_type: inode_type,
            start_block: start_block,
            size: size,
            c_time: c_time,
        }
    }
    //Getters
    pub fn get_inodenum(&self) -> &i32 {
        return &self.inode_num;
    }

    pub fn get_inodetype(&self) -> &InodeType {
        return &self.inode_type;
    }

    pub fn get_start_block(&self) -> &i32 {
        return &self.start_block;
    }

    pub fn get_size(&self) -> &i32 {
        return &self.size;
    }

    pub fn get_ctime(&self) -> &DateTime<Utc> {
        return &self.c_time;
    }

    //Setters
    pub fn set_inodenum(&mut self, inodeNum: i32) -> &mut i32 {
        &mut self.inode_num
    }

    pub fn set_inodetype(&mut self, inode_type: InodeType) -> &mut InodeType {
        &mut self.inode_type
    }

    pub fn set_startblock(&mut self, startBlock: i32) -> &mut i32 {
        &mut self.start_block
    }

    pub fn set_size(&mut self, size: i32) -> &mut i32 {
        &mut self.size
    }
    pub fn set_cTime(&mut self, c_time: DateTime<Utc>) -> &mut DateTime<Utc> {
        &mut self.c_time
    }

    /*pub fn to_json(&mut self) -> Result<()> {
        let mut res = &self;
        let serialized_block = serde_json::to_string_pretty(res)?; //.unwrap()
        println!("{}", serialized_block);
        Ok(())
    }*/
    pub fn to_json(&self) -> String {
        let serialized_block = serde_json::to_string(&self).unwrap();

        return String::from(serialized_block);
    }

    pub fn from_json(source: String) -> Inode {
        let inode: Inode = serde_json::from_str(&source).unwrap();
        println!("{:?}", inode);
        return inode;
    }
}

impl Inodes {
    /**
     * Returns a new inode vec array with specified parameters based on size.
     */
    pub fn new(&mut self, size: usize) -> Inodes {
        let mut inodes: Vec<Inode> = Vec::with_capacity(size);
        Inodes { inodes: inodes }
    }

    /**
     * Returns a default inode vec.
     */
    pub fn default() -> Inodes {
        let mut inodes: Vec<Inode> = Vec::new();

        Inodes { inodes: inodes }
    }

    /**
     * Returns a new inode vec with a copy of another inode vec.
     */
    pub fn new_of_vec(new_inodes: Vec<Inode>) -> Inodes {
        let mut inodes: Vec<Inode> = Vec::with_capacity(new_inodes.len());

        for i in 0..new_inodes.len() {
            inodes.push(new_inodes[i].clone());
        }

        Inodes { inodes: inodes }
    }

    pub fn get_inodes(&self) -> Vec<Inode> {
        let mut inodesCopy: Vec<Inode> = Vec::with_capacity(self.inodes.len().clone());

        for i in 0..self.inodes.len() {
            inodesCopy.push(self.inodes[i].clone());
        }

        return inodesCopy;
    }

    pub fn setInodes(&mut self, new_inodes: Vec<Inode>) {
        self.inodes = Vec::with_capacity(new_inodes.len());
        for i in 0..new_inodes.len() {
            self.inodes.push(new_inodes[i].clone());
        }
    }

    pub fn to_json(&self) -> String {
        let serialized_block = serde_json::to_string(&self).unwrap();

        return String::from(serialized_block);
    }

    pub fn from_json(source: String) -> Inodes {
        let inodes: Inodes = serde_json::from_str(&source).unwrap();
        return inodes;
    }
}
