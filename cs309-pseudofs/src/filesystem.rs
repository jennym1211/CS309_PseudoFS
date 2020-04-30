
use crate::inode::Inode;
use crate::disk::Disk;
use crate::disk::block::Block;
use crate::superblock::Superblock;
use crate::directory::Directory;
use std::fs::File;

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

    /**
	 * Prints diagnostic and usage information about the mounted file system
	 */
    pub fn diagnostics(&self)
    {
        if self.disk.mounted == true && self.mounted == true //if disk is mounted and filesystem is mounted
        {
            let mut diagnostics = String::from("File system magic number is: ");

            if(self.mounted == true)
            {
                diagnostics.push_str("Valid \n");
            }
            else
            {
                diagnostics.push_str("Invalid \n");
            }

            diagnostics.push_str("\t Reads: ");
            diagnostics.push_str(&self.disk.reads.to_string());
            diagnostics.push_str("\t Writes: ");
            diagnostics.push_str(&self.disk.writes.to_string());
            diagnostics.push_str("\n \t Inodes: ");
            diagnostics.push_str(&self.superblock.total_inodes.to_string());
            //diagnostics.push_str("\t \t Free Inodes: ");
            //diagnostics.push_str(&self.superblock.total_inodes.to_string());
            //diagnostics.push_str("\t \t Used Inodes: ");
            diagnostics.push_str("\n \t Blocks: ");
            diagnostics.push_str(&self.superblock.total_blocks.to_string());
             //diagnostics.push_str("\t \t Free Blocks: ");
            //diagnostics.push_str(&self.superblock.total_inodes.to_string());
            //diagnostics.push_str("\t \t Used Blocks: ");
            
          println!("{}", diagnostics );
        }
        else
        {
            println!("File System not mounted. No diagnostics available.");
        }
    }


    pub fn create(fileName: String) -> bool {

        let mut file = File::create(fileName);


        return false;
    }
    
    pub fn format(fileName: String) -> bool {

        let magic_number = String::from("0x70736575646F4653");
        let mut total_blocks : u32 = 0;
        let mut total_inodes : u32 = 0;
        let mut free_blocks_vec: Vec<u32>= Vec::new();
        let mut free_inodes_vec: Vec<u32> = Vec::new();
        let mut super_block = Superblock
        {
             magic_number: magic_number, // 0x70736575646F4653 magic number
            total_blocks: total_blocks,
            free_blocks: free_blocks_vec,
            total_inodes: total_inodes,
            free_inodes: free_inodes_vec
        };


        return true;
    }
    
    pub fn mount(file_name: String) -> bool {
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