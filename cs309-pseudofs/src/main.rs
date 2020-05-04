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
                    let mut path_name = "cs309-pseudofs/disks/disk2.disk";
                    path_name.trim_matches(&['\n', '\r'] as &[_]);



                    println!("{:?}", path_name.to_string());    
                    fs.create_disk(path_name.to_string(), size);
                      
                }
                "format" => {
                    println!("Enter the disk image name.");
             
                    let mut path_name = "./disks/disk2.disk";
                    path_name.trim_matches(&['\n', '\r'] as &[_]);


                    if fs.format(path_name.to_string()) {
                        println!("Formatting successful!");
                    } else {
                        eprintln!("There was a problem formatting your disk.");
                    }
                    break;
                }
                "mount" => {

                    let mut path_name = "./disks/disk.disk";
                    path_name.trim_matches(&['\n', '\r'] as &[_]);


                    print!("{:?}", path_name.to_string());
                    
                    
                    if fs.mount(path_name.to_string()) {
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
                        eprintln!("There was a problem unmounting your disk.");
                    }
                    break;
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
                    println!("These are the following commands within the PseudoFS: 
                        \n 1. create \n This will create a new disk image. 
                        \n 2. format \n This will format the disk image you input. 
                        \n 3. mount \n This will mount the disk you specify.
                        \n 4. unmount \n This will unmount the disk you specify.
                        \n 5. diagnostics \n This will print diagnostics about the current disk mounted.
                        \n 6. delete \n This will delete the disk image you input.
                        \n 7. cat \n This display the file contents of the final name you input.
                        \n 8. ls \n This will list all current files in the directory.
                        \n 9. copyin \n This will copy in a file to the directory from your PC
                        \n 10. copyout \n This will copy a file from the directory to your PC. 
                        \n 11. help \n This will display the commands currently within the PseudoFS.
                        \n 12. exit \n This will unmount your disk image and exit the file system.");
                    break;
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
