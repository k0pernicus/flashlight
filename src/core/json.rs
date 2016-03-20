use core::indexing::IndexedDocuments;
use rustc_serialize::json;
use std::fs::File;
use std::io::prelude::*;

pub fn save_indexed_documents(indexed_documents: &IndexedDocuments, filepath: &str) {
    let data_to_save = export_data(indexed_documents);
    let buffer = File::create(filepath);
    match buffer {
        Ok(mut buffer_rst) => {
            match buffer_rst.write_all(data_to_save.as_bytes()) {
                Ok(_) => println!("Data has been saved!"),
                Err(error) => panic!("ERROR: Data canno't be saved - {}", error),
            }
        },
        Err(error) => panic!("ERROR: File canno't be created - {}", error),
    }
}

pub fn import_indexed_documents(filepath: &str) -> IndexedDocuments {
    import_data(filepath)
}

/// Export data from an object 'IndexedDocuments' to a String
fn export_data(main_doc: &IndexedDocuments) -> String {
    let encoded = json::encode(main_doc);
    match encoded {
        Ok(value) => value,
        Err(_) => panic!("ERROR"),
    }
}

/// Import data from a string to an IndexedDocuments
fn import_data(main_doc: &str) -> IndexedDocuments {
    let file_pointer = File::open(main_doc);
    match file_pointer {
        Ok(mut file) => {
            let mut file_content = String::new();
            match file.read_to_string(&mut file_content) {
                Ok(_) => json::decode(&file_content).unwrap(),
                Err(error) => panic!("ERROR: canno't read to string the file content - {}", error),
            }
        },
        Err(error) => panic!("ERROR: canno't import data - {}", error),
    }
}
