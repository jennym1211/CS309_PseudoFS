use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Inode {
    pub inode_num: u32,
    pub inode_type: InodeType,
    pub start_block: u32,
    pub size: u32,
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
    pub fn new(
        inode_num: u32,
        inode_type: InodeType,
        start_block: u32,
        size: u32,
        c_time: DateTime<Utc>,
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
    pub fn get_inodenum(&self) -> &u32 {
        return &self.inode_num;
    }

    pub fn get_inodetype(&self) -> &InodeType {
        return &self.inode_type;
    }

    pub fn get_start_block(&self) -> &u32 {
        return &self.start_block;
    }

    pub fn get_size(&self) -> &u32 {
        return &self.size;
    }

    pub fn get_ctime(&self) -> &DateTime<Utc> {
        return &self.c_time;
    }

    //Setters
    fn set_inodenum(&mut self) -> &mut u32 {
        &mut self.inode_num
    }

    fn set_inodetype(&mut self) -> &mut InodeType {
        &mut self.inode_type
    }

    fn set_startblock(&mut self) -> &mut u32 {
        &mut self.start_block
    }

    fn set_size(&mut self) -> &mut u32 {
        &mut self.size
    }
    fn set_cTime(&mut self) -> &mut DateTime<Utc> {
        &mut self.c_time
    }

    pub fn toJSON(&self) {
        let serialized_block = serde_json::to_string(&self).unwrap();

        println!("{}", serialized_block);
    }

    pub fn fromJSON(source: String) -> Inode {
        let json_string = "{\"inode_num\":temp,\"inode_type\":\"temp\",\"start_block\":\"temp\",\"size\":temp,\"c_time\":temp}}";

        let inode: Inode = serde_json::from_str(&json_string).unwrap();
        return inode;
    }
}
