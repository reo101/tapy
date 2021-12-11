use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, ListBox, PolicyType, ScrolledWindow};
// use gtk::gdk_pixbuf::prelude::*;

pub fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .build();

    // Create a `ListBox` and add labels with integers from 0 to 100
    let list_box = ListBox::new();
    (0_u32..=100)
        .into_iter()
        .for_each(|num| list_box.append(&Label::new(Some(&num.to_string()))));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_box)
        .build();
    window.set_child(Some(&scrolled_window));
    window.show();
}

// pub fn build_ui(application: &Application) {
//     let window = ApplicationWindow::new(application);
//
//     window.set_title(Some("First GTK Program"));
//     window.set_default_size(350, 70);
//     let scrollable_window: ScrolledWindow = ScrolledWindow::builder()
//         .min_content_height(50)
//         .max_content_height(500)
//         .min_content_width(100)
//         .max_content_width(150)
//         .visible(true)
//         .window_placement(CornerType::TopLeft)
//         .build();
//
//     window.set_child(Some(&scrollable_window));
//
//     // let button = gtk4::Button::with_label("Click me!");
//     //
//     // window.set_child(Some(&button));
//
//     window.show();
// }
