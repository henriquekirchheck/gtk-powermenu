use std::process::Command;

use gtk::{
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, Box, Button
};

const APP_ID: &str = "com.henriquekirchheck.gtk-powermenu";
const APP_TITLE: &str = "GTK Power Menu";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn shutdown(reboot: bool) -> Result<(), String> {
    let mut shutdown_cmd = Command::new("shutdown");
    shutdown_cmd.arg(if reboot {"-r"} else {"-p"}).arg("now");
    match shutdown_cmd.output() {
        Ok(output) => {
            if output.status.success() {
                return Ok(());
            }
            return Err("Failed to execute shutdown command".to_owned());
        }
        Err(error) => return Err(error.to_string()),
    }
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .default_width(300)
        .default_height(100)
        .build();

    let gtk_box = Box::builder()
        .margin_bottom(12)
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let shutdown_button = Button::builder()
        .label(" Shutdown")
        .margin_start(2)
        .margin_end(2)
        .build();

    shutdown_button.connect_clicked(clone!(@weak window => move |_| {
        window.close();
        shutdown(false).expect("A error happend and shutdown was not successfull");
    }));
    shutdown_button.set_hexpand(true);

    let reboot_button = Button::builder()
        .label(" Reboot")
        .margin_start(2)
        .margin_end(2)
        .build();
    reboot_button.set_hexpand(true);

    reboot_button.connect_clicked(clone!(@weak window => move |_| {
        window.close();
        shutdown(true).expect("A error happend and reboot was not successfull");
    }));

    gtk_box.append(&shutdown_button);
    gtk_box.append(&reboot_button);

    window.set_child(Some(&gtk_box));
    window.connect_destroy(clone!(@weak app => move |_| {
        app.quit();
    }));
    window.present();
}
