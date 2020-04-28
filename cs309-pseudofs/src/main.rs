use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Superblock {
    magic_number: String, // 0x70736575646F4653 magic number
    total_blocks: u32,
    free_blocks: Vec<u32>,
    total_inodes: u32,
    free_inodes: Vec<u32>,
}

pub struct Inode {
    inode_num: u32,
    inode_type: InodeType,
    start_block: u32,
    size: u32,
    c_time: String, //find a date/time datastruct idk
}

pub enum InodeType {
    Free,
    File,
    Directory,
    Symlink,
}

pub struct Block {
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
    file: PseudoFile,
    blocks: u32,
    reads: u128,
    writes: u128,
    mounted: bool,
}

//Disk emulator functions
impl DiskImage {
    fn open(file: PseudoFile) -> bool {
        return true;
    }

    fn close(disk: DiskImage) -> bool {
        return true;
    }

    fn read(disk: DiskImage, blockID: u32) -> Block {
        //To:DO implement
        let temp_block = Block {
            nextNode: 0,
            data: String::from("Temp data"),
        };

        return temp_block;
    }

    pub fn write(disk: DiskImage, blockID: u32, block: Block) -> bool {
        return true;
    }
}

/*
Utilized code from : https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
and https://tjtelan.com/blog/building-a-unix-shell-in-rust-part-4/

Shell functions
*/
struct Command {
    keyword: String,
    args: Vec<String>,
}

enum Commands {
    Create,
    Format,
    Unmount,
    Mount,
    Delete,
    Cat,
    Ls,
    Copyin,
    Copyout,
    Help,
    Exit,
}

impl FromStr for Commands {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "create" => Ok(Commands::Create),
            "format" => Ok(Commands::Format),
            "unmount" => Ok(Commands::Unmount),
            "mount" => Ok(Commands::Mount),
            "delete" => Ok(Commands::Delete),
            "cat" => Ok(Commands::Cat),
            "ls" => Ok(Commands::Ls),
            "copyin" => Ok(Commands::Copyin),
            "copyout" => Ok(Commands::Copyout),
            "help" => Ok(Commands::Help),
            "exit" => Ok(Commands::Exit),
            _ => Err(()),
        }
    }
}

/*fn process_command(c: Command) -> i32 {
    //need to parse command out to get information

    match Commands::from_str(&c.keyword) {
        Ok(Commands::Create) => create(),
        Ok(Commands::Format) => format(),
        Ok(Commands::Unmount) => unmount(),
        Ok(Commands::Mount) => mount(),
        Ok(Commands::Delete) => delete(),
        Ok(Commands::Cat) => cat(),
        Ok(Commands::Ls) => ls(),
        Ok(Commands::Copyin) => copyin(),
        Ok(Commands::Copyout) => copyout(),
        Ok(Commands::Help) => help(),
        Ok(Commands::Exit) => exit(),
        _ => {
            println!("{}: command not found", &c.keyword);
            1
        }
    }
}*/

//File system commands
pub fn create(mut file: PseudoFile) -> DiskImage {
    DiskImage {
        file: file,
        blocks: 1024,
        reads: 0,
        writes: 0,
        mounted: false,
    }
}

pub fn format(file: PseudoFile) -> bool {
    return true;
}

pub fn mount(file: PseudoFile) -> bool {
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

//Shell commands
pub fn delete() {}

pub fn cat() {}

pub fn ls() {}

pub fn copyin() {}

pub fn copyout() {}

pub fn help() {}

pub fn exit() {
    println!("Exiting shell...please wait");
}

/*
Utilized code from : https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
*/
pub fn shell() {}

fn main() {
    println!("Hello, world!");
}
