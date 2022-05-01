use gtk::{
    traits::{ButtonExt, ContainerExt, WidgetExt},
    Align, Button, InputPurpose, Orientation,
};

use crate::handlers::email_handler::SmtpConnectionManager;

pub fn login() -> gtk::Box {
    let parent = gtk::Box::new(Orientation::Vertical, 1);
    let email = gtk::Entry::builder()
        .halign(Align::Center)
        .placeholder_text("Example@example.com")
        .margin_top(10)
        .width_request(400)
        .input_purpose(InputPurpose::Email)
        .build();
    email.show_all();

    let password = gtk::Entry::builder()
        .halign(Align::Center)
        .placeholder_text("Password")
        .margin_top(5)
        .margin_bottom(2)
        .width_request(400)
        .input_purpose(InputPurpose::Password)
        .visibility(false)
        .build();
    password.show_all();
    let smpt_domain = gtk::Entry::builder()
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
    // TODO grab actual string from Entry
    let user = email.clone().to_string();
    let pass = password.clone();
    submit_btn.connect_clicked(move |_| {
        println!("{} {}", user, pass);
        // let mut manager = SmtpConnectionManager::new("smtp.gmail.com", &user, &pass);
        // println!("{}", manager.email_addr);
    });

    parent.add(&email);
    parent.add(&password);
    parent.add(&smpt_domain);
    parent.add(&submit_btn);
    parent.set_height_request(600);
    parent.set_width_request(500);
    parent
}
