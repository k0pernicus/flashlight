use std::process::Command;

/// Open a file with the ```open``` command system
fn open_file(file : &str) {

    let command_to_execute = "xdg-open".to_string() + file;

    match Command::new(command_to_execute).status() {
        Ok(_) => (),
        Err(e) => println!("failed to execute process: {}", e),
    }

}
