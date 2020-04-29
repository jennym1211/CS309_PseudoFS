#[derive(Debug, PartialEq, Clone)]
pub struct Superblock {
   pub magic_number: String, // 0x70736575646F4653 magic number
   pub total_blocks: u32,
   pub free_blocks: Vec<u32>,
   pub total_inodes: u32,
   pub  free_inodes: Vec<u32>
}

impl Superblock
{

}


