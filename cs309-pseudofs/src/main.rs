use std::io::{stdin, stdout, Write};
mod directory;
mod disk;
mod filesystem;
mod inode;
mod superblock;
use filesystem::FileSystem;

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
        stdin().read_line(&mut input).unwrap();

        let size = 1024;

        let mut commands = input.trim().split(" | ").peekable();

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            match command {
                "create" => {
                    // println!("Enter the disk image name.");
                    let path_name = "cs309-pseudofs/disks/disk2.disk";
                    path_name.trim_matches(&['\n', '\r'] as &[_]).is_ascii();

                    println!("{:?}", path_name.to_string());
                    fs.create_disk(path_name.to_string(), size);
                }
                "format" => {
                    // println!("Enter the disk image name.");
                    let path_name = "./disks/disk2.disk";
                    path_name.trim_matches(&['\n', '\r'] as &[_]).is_ascii();

                    if fs.format(path_name.to_string()) {
                        println!("Formatting successful!");
                    } else {
                        eprintln!("There was a problem formatting your disk.");
                    }
                    break;
                }
                "mount" => {
                    let path_name = "./disks/diskc.disk";
                    path_name.trim_matches(&['\n', '\r'] as &[_]).is_ascii();

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
                    let path_name = "./disks/disk5.disk";
                    path_name.trim_matches(&['\n', '\r'] as &[_]).is_ascii();

                    if fs.delete_file(path_name.to_string()) {
                        println!("File deletion successful!");
                    } else {
                        eprintln!("There was a problem deleting your file.");
                    }
                    break;
                }
                "cat" => {
                    let path_name = "./disks/disk2.disk";
                    path_name.trim_matches(&['\n', '\r'] as &[_]).is_ascii();

                    fs.read_file(path_name.to_string());
                    break;
                }
                "ls" => {
                    fs.root.list();
                    break;
                }
                "copyin" => {
                    fs.copy_in().expect("Could not write file in.");
                    break;
                }
                "copyout" => {
                    fs.copy_out().expect("Could not write file out.");
                    break;
                }
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
