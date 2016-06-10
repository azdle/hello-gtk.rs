extern crate gtk;
extern crate hyper;

use gtk::prelude::*;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    gtk::init();

    // Create Top Level Window
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    // Setup Window Stuff
    window.set_title("Exosite Timestamp Reader Thing");
    window.set_border_width(10);
    //window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    // Create UI Elements
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let label = gtk::Label::new(Some("Click Update, I Dare You!"));
    let button = gtk::Button::new_with_label("Update");

    // Connect Up Layout
    vbox.add(&label);
    vbox.add(&button);
    window.add(&vbox);

    // Setup Button Click Handler
	button.connect_clicked(move |_| {
        label.set_text(&get_time())
    });

	// Show and Run
    window.show_all();
    gtk::main();
}



fn get_time() -> String {
    // Create a client.
    let mut client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://m2.exosite.com/timestamp")
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    body
}