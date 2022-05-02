use glib::prelude::*;
use gio::prelude::*;
use gtk::{
    traits::{ButtonExt, ContainerExt, WidgetExt, EntryExt},
    Align, Box, Button, Entry, InputPurpose, Orientation,
};

use crate::handlers::email_handler::SmtpConnectionManager;

pub fn login() -> gtk::Box {
    let parent = Box::new(Orientation::Vertical, 1);
    let email = Entry::builder()
        .halign(Align::Center)
        .placeholder_text("Example@example.com")
        .margin_top(10)
        .width_request(400)
        .input_purpose(InputPurpose::Email)
        .build();
    email.show_all();

    let password = Entry::builder()
        .halign(Align::Center)
        .placeholder_text("Password")
        .margin_top(5)
        .margin_bottom(2)
        .width_request(400)
        .input_purpose(InputPurpose::Password)
        .visibility(false)
        .build();
    password.show_all();
    let smpt_domain = Entry::builder()
        .halign(Align::Center)
        .placeholder_text("SMPT domain")
        .margin_top(10)
        .width_request(400)
        .input_purpose(InputPurpose::Url)
        .build();
    smpt_domain.show_all();

    let submit_btn = Button::builder()
        .width_request(200)
        .label("Login")
        .margin_start(50)
        .margin_end(50)
        .margin_top(4)
        .build();
    submit_btn.show_all();
        
    parent.add(&email);
    parent.add(&password);
    parent.add(&smpt_domain);
    parent.add(&submit_btn);
    parent.set_height_request(600);
    parent.set_width_request(500);
    

    let input_pass = password.clone();
    let input_email = email.clone();
    let input_domain = smpt_domain.clone();
    submit_btn.connect_clicked(move |_| {
        println!("Stuff: {} {} {}", &input_pass.text().as_str(), &input_email.text().as_str(), &input_domain.text().as_str());
    });
    parent
}
