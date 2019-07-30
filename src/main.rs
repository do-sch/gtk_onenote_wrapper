#[macro_use]
mod macros;
mod app;
mod window;
mod res;
mod ui;

use std::env::args;
use std::process;

fn main() {
    gtk::init().expect("Failed to start GTK. Please install GTK3.");
    match app::App::new() {
        Err(e) => {
            eprintln!("Error while registering GTK3 application: {:?}", e);
            process::exit(1);
        }
        Ok(app) => {
            app.run(&args().collect::<Vec<_>>());
        }
    }
}