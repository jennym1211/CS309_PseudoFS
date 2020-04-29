use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

mod superblock;
mod inode;
mod disk;
mod filesystem;
mod directory;
use inode::Inode;
use disk::Disk;
use disk::block::Block;
use filesystem::FileSystem;

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
