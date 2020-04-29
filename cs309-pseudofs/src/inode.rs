
#[derive(Debug, PartialEq, Clone)]
pub struct Inode {
    pub inode_num: u32,
    pub inode_type: InodeType,
    pub start_block: u32,
    pub size: u32,
    pub c_time: String, //find a date/time datastruct idk
}

#[derive(Debug, PartialEq, Clone)]
pub enum InodeType {
    Free,
    File,
    Directory,
    Symlink,
}

