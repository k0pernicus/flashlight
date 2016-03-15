extern crate argparse;
extern crate walkdir;

mod core;
mod gui;

use argparse::{ArgumentParser, Store, StoreTrue};
use core::indexing::IndexedDocuments;
use core::searching::search_file_in_db;
use gui::search_bar::create_gui;
use std::io;
use std::io::{Write};

// TODO: NO CASE MATCHING!!!!!

fn main() {

    let mut indexation = String::new();
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
        // Parse those arguments
        ap.parse_args_or_exit();
    }

    create_gui();

    if indexation != "" {

        let mut main_doc = IndexedDocuments::new(indexation);
        // main_doc.set_verbose_mod(true);
        main_doc.begin_indexation();

        loop {

            let mut input = String::new();

            print!(">>> ");
            io::stdout().flush().unwrap();

            match io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(error) => println!("error: {}", error),
            }

            input = input.trim().to_string();

            search_file_in_db(&main_doc, &input);

        }

    }

}
