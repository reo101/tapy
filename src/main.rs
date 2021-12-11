use gtk::{gio::SimpleAction, glib, prelude::*, Application};

mod entry_completion;
mod scrolled_window;
mod media;

fn main() {
    let application = Application::builder()
        .application_id("bg.reo101.tapy")
        .build();
    application.connect_activate(scrolled_window::build_ui);

    // When activated, shuts down the application
    let quit = SimpleAction::new("quit", None);
    quit.connect_activate(
        glib::clone!(@weak application => move |_action, _parameter| {
            application.quit();
        }),
    );
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
    application.add_action(&quit);

    // Run the application
    application.run();
}
