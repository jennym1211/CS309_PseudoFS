
use crate::inode::Inode;
use crate::disk::Disk;
use crate::disk::block::Block;
use crate::superblock::Superblock;
use crate::directory::Directory;


#[derive(Debug, PartialEq, Clone)]
pub struct FileSystem
{
    pub disk: Disk,
    pub superblock: Superblock,
    pub directory: Directory,
    pub inodes: Vec<Inode>,
    pub inodes_per_block: u32,
    pub mounted: bool

}

impl FileSystem

{
    pub fn create(mut file: Vec<String>) -> Disk {
        Disk {
            file: file,
            blocks: 1024,
            reads: 0,
            writes: 0,
            mounted: false,
        }
    }
    
    pub fn format(file: Vec<String>) -> bool {
        return true;
    }
    
    pub fn mount(file: Vec<String>) -> bool {
        return true;
    }
    
    pub fn unmount() -> bool {
        return true;
    }
    
    pub fn readInode(inode_number: u32) {}
    
    pub fn writeInode(inode_num: u32, updated_inode: Inode) -> bool {
        return true;
    }
    
    pub fn getFreeInode() {}
    
    pub fn readBlock(blockID: u32) {}
    
    pub fn writeBlock(blockID: u32, block: Block) -> bool {
        return true;
    }
    
    pub fn getFreeBlock() {}
}