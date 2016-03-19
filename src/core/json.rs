use core::indexing::IndexedDocuments;
use rustc_serialize::json;
use std::fs::File;
use std::io::prelude::*;

/// Export data from an object 'IndexedDocuments' to a String
pub fn export_data(main_doc: &IndexedDocuments) -> String {
    let encoded = json::encode(main_doc);
    match encoded {
        Ok(value) => value,
        Err(_) => panic!("ERROR"),
    }
}

/// Import data from a string to an IndexedDocuments
fn import_data(main_doc: &str) -> IndexedDocuments {
    let mut file_pointer = File::open(main_doc);
    match file_pointer {
        Ok(mut file) => {
            let mut file_content = String::new();
            file.read_to_string(&mut file_content);
            json::decode(&file_content).unwrap()
        },
        Err(error) => panic!("Error to import data: {}", error),
    }
}
