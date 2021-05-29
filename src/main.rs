#[cfg(target_os = "windows")]
use native_dialog::{FileDialog, MessageDialog, MessageType};

use rayon::prelude::*;
use std::env;
use std::io;

mod lib;

fn main() {
    let dict_path: String = match env::var("RWORDFINDER_DICT") {
        Ok(path) => path,

        Err(_) => path_getter(),
    };

    let wordsearch_path = match env::var("RWORDFINDER_WSEARCH") {
        Ok(path) => path,

        Err(_) => path_getter(),
    };

    println!("Allocating data");

    let to_solve = lib::WordBlob::alloc(&wordsearch_path, &dict_path);

    println!("Starting execution");
    
    let results: Vec<_> = (0..to_solve.wordsearch.len())
        .into_par_iter()
        .map(|x| lib::WordBlob::start(&to_solve, x))
        .collect();

    for result in results {
        match result {
            Some(v) => {
                for word in v {
                    println!("Word {} found at {:?} going {}", word.0, word.2, word.1)
                }
            }
            None => (),
        }
    }
}

//get path if env variable isn't set
fn path_getter() -> String {
    if cfg!(target_os = "windows") {
        let path = FileDialog::new()
            .set_location(".")
            .add_filter("Text file", &["txt"])
            .show_open_single_file()
            .unwrap();

        let path = match path {
            Some(path) => path,
            None => panic!("The file has not been selected"),
        };

        let yes = MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Do you want to open the file?")
            .set_text(&format!("{:#?}", path))
            .show_confirm()
            .unwrap();

        if yes {
            path.into_os_string()
                .into_string()
                .expect("Improper encoding")
        } else {
            panic!("Idk what to do here tbh")
        }

    } else {
        println!("Please write out a path to the dictionary");

        let mut path = String::new();

        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read line");

        path
    }
}
