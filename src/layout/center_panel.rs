use gtk::{
    traits::{ContainerExt, WidgetExt},
    Align, Button, InputPurpose, Orientation,
};

pub fn center_box() -> gtk::Box {
    let parent = gtk::Box::new(Orientation::Vertical, 1);
    let email = gtk::Entry::builder()
        .halign(Align::Center)
        .placeholder_text("Example@example.com")
        .margin_top(10)
        .width_request(200)
        .input_purpose(InputPurpose::Email)
        .build();
    email.show_all();

    let password = gtk::Entry::builder()
        .halign(Align::Center)
        .placeholder_text("Password")
        .margin_top(5)
        .margin_bottom(2)
        .width_request(200)
        .input_purpose(InputPurpose::Password)
        .visibility(false)
        .build();
    password.show_all();

    let submit_btn = Button::builder().width_request(200).label("Submit").build();
    submit_btn.show_all();

    parent.add(&email);
    parent.add(&password);
    parent.add(&submit_btn);
    parent.set_height_request(600);
    parent.set_width_request(500);
    parent
}
