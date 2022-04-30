use crate::layout::center_panel;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
pub fn run() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(420)
            .default_height(700)
            .title("Rust mail")
            .build();

        // Don't forget to make all widgets visible.
        win.add(&center_panel::center_box());
        win.show_all();
    });

    app.run();
}
