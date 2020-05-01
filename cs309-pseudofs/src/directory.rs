use crate::inode::Inode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Directory {
    pub directory_contents: HashMap<String, u32>,
}

/*pub struct Directory {
    pub name: String,
    pub inode: Inode,
    pub file_name: String,
}*/

impl Directory {
    pub fn new() -> Directory {
        let mut contents: HashMap<String, u32> = HashMap::new();

        contents.clear();

        contents.insert(String::from("."), 0);
        contents.insert(String::from(".."), 0);
        contents.insert(String::from("/"), 0);

        let mut directory = Directory {
            directory_contents: contents,
        };

        return directory;
    }

    pub fn remove(&mut self, name: String) {
        if !name.eq("/") || name.eq(".") || name.eq("..")
        //cannot remove root
        {
            self.directory_contents.remove(&name);
        } else {
            println!("Cannot remove this directory!");
        }
    }

    /*

        Returns a vec of all the files in the directory, sorted.

    */
    pub fn getContents(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        let mut i = 0;

        // do for loop
        return names;
    }
    /*

       Adds a file to the directory

    */
    pub fn add(&mut self, inode_num: u32, name: String) -> bool {
        if inode_num > 0 {
            self.directory_contents.insert(name, inode_num);
            return true;
        }
        return false;
    }

    pub fn toJSON(&self) {
        let serialized_block = serde_json::to_string(&self).unwrap();

        println!("{}", serialized_block);
    }

    pub fn fromJSON(source: String) -> Directory {
        let directory: Directory = serde_json::from_str(&source).unwrap();
        return directory;
    }
}
