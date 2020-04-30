use serde::{Serialize, Deserialize};
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Inode {
    pub inode_num: u32,
    pub inode_type: InodeType,
    pub start_block: u32,
    pub size: u32,
    pub c_time: String, //find a date/time datastruct idk
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum InodeType {
    Free,
    File,
    Directory,
    Symlink,
}



impl Inode
{
    pub fn toJSON(&self)
   {
       let serialized_block = serde_json::to_string(&self).unwrap();

       println!("{}", serialized_block);
       
   }

   pub fn fromJSON(source: String) -> Inode
   {
       let json_string = "{\"inode_num\":temp,\"inode_type\":\"temp\",\"start_block\":\"temp\",\"size\":temp,\"c_time\":temp}}";

       let inode: Inode = serde_json::from_str(&json_string).unwrap();
       return inode;

   }
}

