use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use gtk4::glib::clone;
use gtk4::glib::signal::Inhibit;
use gtk4::prelude::*;
use lazy_static::lazy_static;

fn main() {
    let application =
        gtk4::Application::new(Some("com.benjaminsproule.rust-gtk4"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

lazy_static! {
    static ref VALUES: Mutex<HashMap<&'static str, bool>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

fn build_ui(application: &gtk4::Application) {
    let checkbox_grid = gtk4::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    let countries = vec!["France", "UK"];
    let check_buttons = countries.iter().map(|country| {
        let mut mutex_guard = VALUES.lock().unwrap();
        mutex_guard.insert(country, false);
        let check_button = gtk4::CheckButton::builder().label(&**country).build();
        check_button.connect_toggled(|check_button| {
            let mut mutex_guard = VALUES.lock().unwrap();
            mutex_guard.insert(country, check_button.is_active());
        });
        check_button
    });
    let mut i = 0;
    for check_button in check_buttons {
        checkbox_grid.attach(&check_button, 0, i, 1, 1);
        i += 1;
    }

    let button = gtk4::Button::builder()
        .label("Open Dialog")
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .build();
    checkbox_grid.attach(&button, 0, i + 1, 1, 1);

    let window = Rc::new(
        gtk4::ApplicationWindow::builder()
            .application(application)
            .title("Country Selector")
            .default_width(350)
            .default_height(70)
            .child(&checkbox_grid)
            .visible(true)
            .build(),
    );

    button.connect_clicked(clone!(@strong window =>
        move |_| {
            gtk4::glib::MainContext::default().spawn_local(dialog(Rc::clone(&window)));
        }
    ));

    window.connect_close_request(move |window| {
        if let Some(application) = window.application() {
            application.remove_window(window);
        }
        Inhibit(false)
    });

    window.show();
}

async fn dialog<W: IsA<gtk4::Window>>(window: Rc<W>) {
    let question_dialog = gtk4::MessageDialog::builder()
        .transient_for(&*window)
        .modal(true)
        .buttons(gtk4::ButtonsType::OkCancel)
        .text(&format!("Selected {:?}", VALUES.lock().unwrap()))
        .build();

    question_dialog.run_future().await;
    question_dialog.close();
}
