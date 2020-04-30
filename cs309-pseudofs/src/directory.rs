use crate::inode::Inode;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Directory {
    pub name: String,
    pub inode: Inode,
    pub file_name: String,
}


impl Directory
{

        //Getters
        pub fn get_name(&self) -> &String {
            return &self.name;
        }

        pub fn get_inode(&self) -> &Inode {
            return &self.inode;
        }
        
        pub fn get_filename(&self) -> &String {
            return &self.file_name;
        }

        //Setters
        fn set_name(&mut self) -> &mut String {
            &mut self.name
        }

        fn set_inode(&mut self) -> &mut Inode {
            &mut self.inode
        }

        fn set_fileName(&mut self) -> &mut String {
            &mut self.file_name
        }


            pub fn toJSON(&self)
        {
            let serialized_block = serde_json::to_string(&self).unwrap();

            println!("{}", serialized_block);
            
        }

        pub fn fromJSON(source: String) -> Directory
        {
            let json_string = "{\"name\":temp,\"inode\":\"temp\",\"file_name\":\"temp\"}}";

            let directory: Directory = serde_json::from_str(&json_string).unwrap();
            return directory;

        }




}