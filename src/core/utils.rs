use std::process::Command;
use std::process::Stdio;

/// Open a file with the ```open``` command system
pub fn open_file(file : &str) {

    match Command::new("xdg-open").arg(file).stdout(Stdio::null()).stderr(Stdio::null()).status() {
        Ok(_) => (),
        Err(e) => println!("failed to execute process: {}", e),
    }

}
