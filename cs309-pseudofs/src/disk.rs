use block::Block;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;


    #[derive(Debug, PartialEq, Clone)]
    pub struct Disk {
       pub file: Vec<String>,
       pub blocks: u32,
       pub reads: u128,
       pub writes: u128,
       pub mounted: bool,
    }
    
    //Disk emulator functions
    impl Disk {

        

        fn open(file_name: String) -> bool {

            return true;
        }
    
        fn close(disk: Disk) -> bool {
            return true;
        }
    
        fn read(disk: Disk, blockID: u32) -> Block {
            
            //need to increment
            let temp_block = Block {
                blockID: 0,
                nextNode: 0,
                data: String::from("Temp data"),
            };
    
            return temp_block;
        }
    
        pub fn write(disk: Disk, blockID: u32, block: Block) -> bool {
            return true;
        }
    }


pub mod block
    {
        #[derive(Debug, PartialEq, Clone)]
     pub struct Block {
        pub blockID: u32, 
        pub nextNode: u32,
        pub data: String,
        }

    }


