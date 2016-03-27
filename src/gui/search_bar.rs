extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;

use core::indexing::IndexedDocuments;
use core::searching::search_file_in_db;
use core::utils::open_file;

use self::gdk::enums::key;
use self::gdk_pixbuf::Pixbuf;
use self::gtk::{Button, Entry, Orientation, Widget, Window, WindowPosition, WindowType};
use self::gtk::prelude::*;

static DEFAULT_ICON: &'static str = "./img/flashlight_icon.png";

// Create rslts
fn create_rslts(rslts: &Vec<String>, vertical_layout: &gtk::Box) {
    clear_rslts(&vertical_layout);
    for rslt in rslts {
        let button = Button::new();
        button.set_label(&format!("{}", rslt));
        button.connect_clicked(|b| {
            open_file(&(b.get_label().unwrap()));
        });
        button.hide_on_delete();
        button.show();
        vertical_layout.add(&button);
    }
    vertical_layout.check_resize();
}

// Clear rslts
fn clear_rslts(vertical_layout: &gtk::Box) {
    for widget in &vertical_layout.get_children() {
        vertical_layout.remove(widget);
        widget.destroy();
    }
}

pub fn run_ui(indexed_documents: IndexedDocuments) {

    // INIT GTK
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let interface = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let default_icon : &Pixbuf = &Pixbuf::new_from_file(DEFAULT_ICON).unwrap();

    // ENTRY
    let entry = Entry::new();
    entry.set_width_chars(50);

    // RESPONSES
    let vertical_layout = gtk::Box::new(Orientation::Vertical, 5);

    interface.pack_start(&entry, true, true, 0);
    interface.pack_start(&vertical_layout, false, true, 0);

    let interface : Widget = interface.upcast();

    // WINDOW
    let window = Window::new(WindowType::Toplevel);
    window.connect_delete_event(|_, _| {
        // Stop the main loop.
        gtk::main_quit();
        // Let the default handler destroy the window.
        Inhibit(false)
    });

    window.set_title("FlashLight");
    window.set_position(WindowPosition::Center);
    // window.set_default_size(1280, 200);

    window.set_icon(Some(default_icon));
    window.add(&interface);
    window.show_all();

    {
        window.connect_key_press_event(move |_, key| {
            match key.get_keyval() as u32 {
                key::Escape => gtk::main_quit(),
                key::Return => {
                    let search_word = entry.get_text().unwrap();
                    println!("Search word: {}", search_word);
                    let rslts = search_file_in_db(&indexed_documents, &search_word);
                    create_rslts(&rslts, &vertical_layout);
                },
                _ => ()
            }
            Inhibit(false)
        });
    }

    // MAIN
    gtk::main();
}
