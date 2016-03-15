extern crate gtk;

use self::gtk::traits::*;
use self::gtk::signal::Inhibit;

pub fn create_gui() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    window.set_title("Flashlight");
    window.set_default_size(1500, 200);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // let button = gtk::Button::new_with_label("Click me!").unwrap();
    let entry = gtk::SearchEntry::new().unwrap();
    entry.set_size_request(800, 80);

    // window.add(&button);
    window.add(&entry);

    window.show_all();
    gtk::main();
}
