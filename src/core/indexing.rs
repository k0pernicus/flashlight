use core::searching::scan_repositories;
use core::utils::open_file;
use std::collections::HashMap;
use std::path::PathBuf;

/// This structure index all documents which are in the filesystem.
#[derive(RustcDecodable, RustcEncodable)]
pub struct IndexedDocuments {
    core : HashMap<String, Vec<Document>>,
    core_vector : Vec<String>,
    paths : Vec<String>,
    root : String,
    verbose_mod : bool,
}

/// This structure represents a file (directory, text, video, audio, binary, ...) in the filesystem.
/// A 'Document' is represented by a name, an extension, a path and a size.
#[derive(RustcDecodable, RustcEncodable)]
pub struct Document {
    extension : String,
    name : String,
    path : String,
    size : u64,
}

impl IndexedDocuments {
    pub fn new() -> IndexedDocuments {
        IndexedDocuments {
            core : HashMap::new(),
            core_vector : Vec::new(),
            paths : Vec::new(),
            root : "".to_string(),
            verbose_mod : false,
        }
    }

    /// Method to modify the position of the verbose mod
    pub fn set_verbose_mod(&mut self, boolean : bool) {
        self.verbose_mod = boolean;
    }

    /// Method to know what is the current position of the verbose mod
    pub fn is_verbose_mod(&self) -> bool {
        self.verbose_mod
    }

    /// Function to sort the paths field
    fn sort_paths_field(&mut self) {
        self.paths.sort();
    }

    pub fn sort_core_vector(&mut self) {
        self.core_vector.sort();
    }

    /// Method to know if a current file has been already indexed or not.
    /// To perform the search, this method uses the binary search.
    pub fn is_path_exists(&mut self, path : &str) -> bool {
        self.sort_paths_field();
        match self.paths.binary_search(&path.to_string()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn is_file_exists(&self, file : &str) -> bool {
        match self.core.get(file) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_paths_from_core(&self, file : &str) -> &Vec<Document> {
        match self.core.get(file) {
            Some(vector_of_documents) => vector_of_documents,
            None => panic!("No found file {} in the core documents", file),
        }
    }

    /// Method to create a document in the core structure
    pub fn create_doc_in_core(&mut self, filename: &str) {
        self.core.insert(filename.to_lowercase(), Vec::new());
    }

    /// Method to add a document to an existing key of the core structure
    pub fn add_doc_in_core(&mut self, filename: &str, document: Document) {
        match self.core.get_mut(&filename.to_lowercase()) {
            Some(doc_core) => {
                doc_core.push(document);
            },
            None => {
                println!("Canno't find {} in core", filename);
            },
        }
    }

    pub fn add_doc_in_core_vector(&mut self, filename: &str) {
        if ! self.core_vector.contains(&filename.to_string()) {
            self.core_vector.push(filename.to_string());
        }
    }

    /// Method to get a reference to the core structure
    pub fn get_core(&self) -> &HashMap<String, Vec<Document>> {
        &self.core
    }

    pub fn get_core_vector(&self) -> &Vec<String> {
        &self.core_vector
    }

    /// Method to get the root indexing node
    pub fn get_root(&self) -> String {
        self.root.clone()
    }

    pub fn set_root(&mut self, root:&str) {
        self.root = root.to_string();
    }

    /// Method to begin the indexation of files
    pub fn begin_indexation(&mut self, root: &str) {
        self.set_root(root);
        scan_repositories(&PathBuf::from(self.get_root()), self);
    }

    /// Method to add a new path (new filepath) in the paths structure
    pub fn add_path(&mut self, new_path: &str) {
        self.paths.push(new_path.to_string());
    }

    pub fn look_after_document(&self, file : &str) {
        if self.is_file_exists(file) {
            if self.verbose_mod {
                println!("Perfect match!");
            }
            for document in self.get_paths_from_core(file) {
                let all_path_file = document.get_path() + "/" + &document.get_filename();
                open_file(&all_path_file);
                if self.verbose_mod {
                    println!("\t* {} -> {} bytes", all_path_file, document.get_size());
                }
            }
        }
        else {
            let corresponding_documents = self.core_vector.iter().filter(|s| s.to_lowercase().starts_with(file)).map(|s| s.to_owned()).collect::<Vec<String>>();
            if corresponding_documents.is_empty() {
                if self.verbose_mod {
                    println!("No available documents for \"{}\"", file);
                }
            }
            else {
                if self.verbose_mod {
                    println!("Available documents for \"{}\":", file);
                }
                for document_filename in corresponding_documents {
                    let documents = self.get_paths_from_core(&document_filename.to_lowercase());
                    for document in documents {
                        if self.verbose_mod {
                            println!("\t {} -> {} bytes", document.get_path() + "/" + &document.get_filename(), document.get_size());
                        }
                    }
                }
            }

        }
    }

}

impl Document {
    pub fn new(extension: &str, name: &str, path: &str, size: u64) -> Document {
        Document {
            extension : extension.to_string(),
            name : name.to_string(),
            path : path.to_string(),
            size : size,
        }
    }

    /// Method to get the filename of the document : name + extension
    pub fn get_filename(&self) -> String {
        if self.extension.len() != 0 {
            self.get_name() + "." + &self.get_extension()
        }
        else {
            self.get_name()
        }
    }

    /// Method to get the extension of the document
    pub fn get_extension(&self) -> String {
        self.extension.clone()
    }

    /// Method to get the name of the document
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Method to get the path of the document
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// Method to get the size of the document
    pub fn get_size(&self) -> u64 {
        self.size
    }

}
