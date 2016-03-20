extern crate argparse;
extern crate rustc_serialize;
extern crate walkdir;

mod core;
mod gui;

use argparse::{ArgumentParser, Store, StoreTrue};
use core::indexing::IndexedDocuments;
use core::json::{save_indexed_documents, import_indexed_documents};
use core::searching::search_file_in_db;
use gui::search_bar::run_ui;
use std::io;
use std::io::prelude::*;

static JSON_FILEPATH: &'static str = "/home/antonin/.indexed_documents.json";

fn main() {

    let mut indexation = String::new();
    let mut visu = "terminal".to_string();
    let mut verbose = false;

    {
        let mut ap = ArgumentParser::new();
        // Set description of the project
        ap.set_description("A search bar, implemented with Rust.");
        // Does the user wants a new indexation ?
        ap.refer(&mut indexation)
            .add_option(&["--indexation"], Store,
            "The root path to begin the indexation of files.");
        // Is the project using the verbose mod ?
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        // Default visualization
        ap.refer(&mut visu)
            .add_option(&["-V", "--visu"], Store,
            "Visualization to use: gui (default) or terminal");
        // Parse those arguments
        ap.parse_args_or_exit();
    }

    let mut main_doc = IndexedDocuments::new();

    if indexation != "" {

        main_doc.begin_indexation(&indexation);
        main_doc.set_verbose_mod(true);

        save_indexed_documents(&main_doc, JSON_FILEPATH);

    }

    else {

        main_doc = import_indexed_documents(JSON_FILEPATH);
        main_doc.set_verbose_mod(true);

    }

    // Terminal visu
    if visu == "terminal" {

        loop {

            let mut input = String::new();

            print!(">>> ");
            io::stdout().flush().unwrap();

            match io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(error) => println!("error: {}", error),
            }

            input = input.trim().to_lowercase();

            search_file_in_db(&main_doc, &input);

        }

    }
    // Let's try the GUI visu
    else {

        run_ui(main_doc)

    }

}
