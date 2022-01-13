use std::process::Command;

use gtk::{
    glib, prelude::*, Application, ApplicationWindow, Box, Button, Label, ListBox, MediaFile,
    Orientation, Picture, PolicyType, ScrolledWindow,
};
use which::which;

use crate::{loaders, utils};

pub fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tapy")
        // TODO: load from config
        .default_width(300)
        .default_height(700)
        .build();

    let outer_container = Box::new(Orientation::Vertical, 50);
    let button = Button::with_label("Copy media");

    // Create a `ListBox` and add Pictures with all media
    let media_list_box = ListBox::new();

    let iter = loaders::media_iter().unwrap(); // FIXME:

    let media_vec = iter
        .map(|path| {
            let tile = Box::new(Orientation::Vertical, 5);
            tile.append(
                &Picture::builder()
                    .paintable(&{
                        let media = MediaFile::for_filename(path.to_str().unwrap_or_else(|| ""));
                        media.set_playing(true);
                        media.set_loop(true);
                        media.set_muted(true);
                        media.set_volume(0.15);
                        media
                    })
                    .height_request(200)
                    .width_request(200)
                    .build(),
            );
            tile.append(&Label::new(Some(
                path.file_name()
                    .map(|os_string| os_string.to_str())
                    .flatten()
                    .unwrap_or(""),
            )));

            tile
        })
        .collect::<Vec<_>>();

    media_vec
        .iter()
        .for_each(|media| media_list_box.append(media));

    button.connect_clicked(
        glib::clone!(@strong media_vec, @strong media_list_box => move |_| {
            media_vec.iter().for_each(|media| {
                match media.orientation() {
                gtk::Orientation::Horizontal => media.set_orientation(gtk::Orientation::Vertical),
                gtk::Orientation::Vertical => media.set_orientation(gtk::Orientation::Horizontal),
                _ => unreachable!(),
                };
            });

            if let Some(selected_row) = media_list_box.selected_row() {
                if let Ok(_) = which("dragon-drag-and-drop") {
                    let selected_file_name = selected_row
                            .first_child().unwrap() // The Box (With Picture and Label)
                            .last_child().unwrap() // The Label
                            .downcast::<Label>().unwrap()
                            .text();

                    Command::new("dragon-drag-and-drop")
                        .current_dir(utils::media_path().unwrap())
                        .arg("--and-exit")
                        .arg(selected_file_name)
                        .output()
                        .expect("Failed to run `dragon-drag-and-drop`");
                } else {
                    panic!("dragon-drag-and-drop binary not found!");
                }

                // Notification::new()
                //     .summary("Firefox News")
                //     .body("This will almost look like a real firefox notification.")
                //     .icon("firefox")
                //     .show()
                //     .unwrap();
            }
        }),
    );

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .vexpand(true)
        .child(&media_list_box)
        .build();

    outer_container.append(&scrolled_window);
    outer_container.append(&button);

    window.set_child(Some(&outer_container));
    window.show();
}
