use crate::inode::Inode;

#[derive(Debug, PartialEq, Clone)]
pub struct Directory {
    pub name: String,
    pub inode: Inode,
    pub file_name: String,
}