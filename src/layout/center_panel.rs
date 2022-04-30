use gtk::{
    traits::{ContainerExt, WidgetExt},
    Align, Button, Orientation, SearchBar,
};

pub fn center_box() -> gtk::Box {
    let parent = gtk::Box::new(Orientation::Horizontal, 1);
    let email = gtk::Entry::builder()
    .halign(Align::Center)
    .margin_top(10)
    .margin_bottom(2)
    .build();
    email.show_all();
    let submit_btn = Button::builder()
        .label("Submit")
        .build();
    submit_btn.show_all();
    parent.add(&email);
    parent.add(&submit_btn);
    parent
}
