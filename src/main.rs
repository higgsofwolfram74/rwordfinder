#[cfg(target_os = "windows")] use native_dialog::{FileDialog, MessageDialog, MessageType};

use std::io;
use std::io::prelude::*;
use std::env;
use rayon::prelude::*;
use crate::WordSearch;

fn main() {
    

    let dict_path: String = match env::var("RWORDFINDER_DICT") {
        Ok(path) => path,
        
        Err(_) => path_getter() 
    };

    let wordsearch_path = match env::var("RWORDFINDER_WSEARCH") {
        Ok(path) => path,

        Err(_) => path_getter()
    };

    let wordsearch = WordBlob::alloc(&wordsearch_path, &dict_path);
}

fn path_getter() -> String {
    if cfg!(target_os = "windows") {
                    
        let path = FileDialog::new()
            .set_location(".")
            .add_filter("Text file", &["txt"])
            .show_open_single_file()
            .unwrap();

        let path = match path {
            Some(path) => path,
            None => panic!("The file has not been selected")
        };

        path

        
        
    } else {
            println!("Please write out a path to the dictionary");

            let mut path = String::new();

            io::stdin()
                .read_line(&mut path)
                .expect("Failed to read line")
    }
}