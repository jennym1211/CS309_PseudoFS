pub struct Superblock {
    magic_number: String, // 0x70736575646F4653 magic number
    total_blocks: u32,
    free_blocks: Vec<u32>,
    total_inodes: u32,
    free_inodes: Vec<u32>,
}

pub struct Inode {
    inode_num: u32,
    inode_type: u32,
    start_block: u32,
    size: u32,
    c_time: String, //find a date/time datastruct idk
}

pub struct DataBlock {
    nextNode: u32,
    data: String,
}

pub struct PseudoFile {
    data: String,
}

pub struct Directory {
    name: String,
    inode: Inode,
    file_name: String,
}

pub struct DiskImage {
    pseudo_file: PseudoFile,
    blocks: u32,
    reads: u128,
    writes: u128,
    mounted: bool,
}

impl DiskImage {}

fn open(image: DiskImage) -> bool {
    return true;
}

fn close(image: DiskImage) -> bool {
    return true;
}

fn main() {
    println!("Hello, world!");
}
