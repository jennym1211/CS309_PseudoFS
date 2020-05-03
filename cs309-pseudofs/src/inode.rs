use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Inode {
    pub inode_num: i32,
    pub inode_type: InodeType,
    pub start_block: i32,
    pub size: i32,
    pub c_time: DateTime<Utc>, //find a date/time datastruct idk
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum InodeType {
    Free,
    File,
    Directory,
    Symlink,
}

impl Inode {
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

    pub fn toJSON(&self) -> String {
        let serialized_block = serde_json::to_string(&self).unwrap();

        return String::from(serialized_block);
    }

    pub fn fromJSON(source: String) -> Inode {
        let inode: Inode = serde_json::from_str(&source).unwrap();
        return inode;
    }
}
