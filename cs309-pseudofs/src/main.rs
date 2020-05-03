use std::env;
use std::io;
use std::io::{stdin, stdout, Write};
use std::num::ParseIntError;
use std::str::FromStr;

use std::process::{Child, Command, Stdio};
mod directory;
mod disk;
mod filesystem;
mod inode;
mod superblock;
use disk::block::Block;
use disk::Disk;
use filesystem::FileSystem;
use inode::Inode;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;
use std::path::Path;

/*
Utilized code from : https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
and https://tjtelan.com/blog/building-a-unix-shell-in-rust-part-4/

Shell functions
*/

/*
Utilized code from : https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
*/
fn main() {
    let mut fs: FileSystem = FileSystem::default();

    println!("Welcome to the Pseudo File System Shell!");

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        let mut stringContents = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut file_name_input = String::new();
        let size = 1024;

        // read_line leaves a trailing newline, which trim removes
        // this needs to be peekable so we can determine when we are on the last command
        let mut commands = input.trim().split(" | ").peekable();

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "create" => {
                    println!("Enter the disk image name.");
                    io::stdin()
                        .read_line(&mut file_name_input)
                        .expect("File not found.");

                    fs.create_disk(file_name_input.clone(), size)
                        .expect("Could not create disk.");
                }
                "format" => {
                    println!("Enter the disk image name.");
                    io::stdin()
                        .read_line(&mut file_name_input)
                        .expect("File not found.");

                    if fs.format(file_name_input) {
                        println!("Formatting successful!");
                    } else {
                        eprintln!("There was a problem formatting your disk.");
                    }
                    break;
                }
                "mount" => {
                    println!("Enter the disk image name to mount.");
                    io::stdin()
                        .read_line(&mut file_name_input)
                        .expect("File not found.");

                    if fs.mount(file_name_input) {
                        println!("Mounting successful!");
                    } else {
                        eprintln!("There was a problem mounting your disk.");
                    }
                    break;
                }
                "unmount" => {
                    if fs.is_mounted().clone() == true && fs.unmount() {
                        println!("Unmounting successful!");
                    } else {
                        eprintln!("There was a problem unmounting your disk");
                    }
                }
                "diagnostics" => {
                    fs.diagnostics();
                    break;
                }
                "delete" => {
                    io::stdin()
                        .read_line(&mut file_name_input)
                        .expect("File not found.");

                    println!("Enter the file name to delete.");

                    if fs.delete_file(file_name_input) {
                        println!("File deletion successful!");
                    } else {
                        eprintln!("There was a problem deleting your file.");
                    }
                    break;
                }
                "cat" => {
                    println!("Enter a file name to display.");
                    io::stdin()
                        .read_line(&mut file_name_input)
                        .expect("File not found.");
                    stringContents = fs.read_file(file_name_input);
                    println!("{}", stringContents);
                    break;
                }
                "ls" => {
                    for content in fs.list() {
                        println!("{:?}", content);
                    }
                    break;
                }
                "copyin" => {
                    println!("Please enter the path to the file on your computer to read.");
                    io::stdin()
                        .read_line(&mut file_name_input.clone())
                        .expect("File not found.");

                    let mut file = File::create(file_name_input.clone());
                    let path = Path::new(&file_name_input);
                }
                "copyout" => {}
                "help" => {
                    println!("Here is a guide on how to use the shell: ");
                }
                "exit" => {
                    println!("Thank you for using the Pseudo File System. Goodbye!");
                    fs.unmount();
                    return;
                }
                _ => println!("Please enter a valid command,"),
            }
        }
    }
}
