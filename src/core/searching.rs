use core::indexing::Document;
use core::indexing::IndexedDocuments;

use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use walkdir::WalkDir;

pub fn search_file_in_db(indexed_documents: &IndexedDocuments, file : &str) -> Vec<String> {
    if file != "" {
        indexed_documents.look_after_document(file)
    }
    else {
        Vec::new()
    }
}

pub fn get_nb_of_child(directory : &PathBuf) -> usize {
    let mut nb_of_child : usize = 1;
    for entry in WalkDir::new(directory).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        let entry_path_str = entry_path.to_str().unwrap();
        let split_document_path = entry_path_str.split("/").collect::<Vec<&str>>();
        if !split_document_path.iter().fold(false, |contains_cached, file| contains_cached || file.starts_with(".")) {
            nb_of_child += 1;
        }
    }
    nb_of_child
}

pub fn scan_repositories(directory : &PathBuf, indexed_documents : &mut IndexedDocuments, nb_of_child : usize) {
    // Get all entries from the PathBuf given as parameter, filter and follow links
    let verbose_mod = indexed_documents.is_verbose_mod();
    let mut current_child = 1;
    for entry in WalkDir::new(directory).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        // Get the parent node
        let entry_path = entry.path();
        let entry_path_str = entry_path.to_str().unwrap();
        // If the file has not been already indexed...
        if ! indexed_documents.is_path_exists(entry_path_str) && ! (entry_path_str == directory.to_str().unwrap()) {

            if verbose_mod {
                println!("ADDED {} -> New document", entry_path_str);
            }

            // Get informations about the document
            let document_size : u64 = entry_path.metadata().unwrap().len();
            let split_document_path = entry_path_str.split("/").collect::<Vec<&str>>();

            // Index only non-cached files
            if ! split_document_path.iter().fold(false, |contains_cached, file| contains_cached || file.starts_with(".")) {

                let document_path = split_document_path[0..split_document_path.len() - 1].join("/");
                let document_filename = split_document_path[split_document_path.len() - 1];
                let document_filename_s = document_filename.split(".").collect::<Vec<&str>>();
                let mut document_extension = "";
                let mut document_filename_without_extension = document_filename.to_string();


                // Just modify the 2 last fields if the length of the ```document_filename_s``` field is different than 1 (no split)
                if document_filename_s.len() != 1 {
                    document_extension = document_filename_s[document_filename_s.len() - 1];
                    document_filename_without_extension = document_filename_s[0..(document_filename_s.len() - 1)].join(".");
                }

                let new_document = Document::new(document_extension,
                    &document_filename_without_extension,
                    &document_path,
                    document_size);

                indexed_documents.add_path(entry_path_str);
                if ! indexed_documents.get_core().contains_key(document_filename) {
                    indexed_documents.create_doc_in_core(document_filename);
                }
                indexed_documents.add_doc_in_core(document_filename, new_document);
                indexed_documents.add_doc_in_core_vector(document_filename);

                print!("\rIndexed files: {0} / {1} files...", current_child, nb_of_child);
                io::stdout().flush().ok().expect("Could not flush stdout");
                current_child += 1;
            }
        }
    }
    indexed_documents.sort_core_vector();
}
