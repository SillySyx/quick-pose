mod session;
mod settings;

use gtk::glib;
use gtk::prelude::*;
use gtk::gdk_pixbuf::{Pixbuf, InterpType};
use gtk::{Application, ApplicationWindow, Builder, FileChooserButton, ComboBox, Stack, Button, Image};

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use settings::Settings;
use session::Session;

enum Page {
    Settings,
    Image,
    Complete,
}

fn set_buttons_visibility(builder: &Builder, back_visible: bool, tools_visible: bool) {
    let back: Button = builder.object("back").expect("Failed to get back");
    let next_image: Button = builder.object("next_image").expect("Failed to get next_image");
    let toggle_pause: Button = builder.object("toggle_pause").expect("Failed to get toggle_pause");
    let previous_image: Button = builder.object("previous_image").expect("Failed to get previous_image");
    let tools: Button = builder.object("tools").expect("Failed to get tools");

    back.set_visible(back_visible);
    next_image.set_visible(tools_visible);
    toggle_pause.set_visible(tools_visible);
    previous_image.set_visible(tools_visible);
    tools.set_visible(tools_visible);
}

fn set_page(builder: &Builder, page: Page) {
    let pages: Stack = builder.object("pages").expect("Failed to get pages");

    match page {
        Page::Settings => pages.set_visible_child_name("page0"),
        Page::Image => pages.set_visible_child_name("page1"),
        Page::Complete => pages.set_visible_child_name("page2"),
    };
}

fn resize_image(builder: &Builder) {
    let pages: Stack = builder.object("pages").expect("Failed to get image_page");
    let image: Image = builder.object("image").expect("Failed to get image");

    if let Some(pixbuf) = image.pixbuf() {
        let allocation = pages.allocation();
        let new_image = pixbuf.scale_simple(allocation.width(), allocation.height(), InterpType::Bilinear).unwrap();
        image.set_pixbuf(Some(&new_image));
    }
}

fn set_image(builder: &Builder, new_image: &str) {
    let image: Image = builder.object("image").expect("Failed to get image");
    let new_image = Pixbuf::from_file(new_image).unwrap();

    image.set_pixbuf(Some(&new_image));

    resize_image(builder);
}

fn build_ui(application: &Application) {
    let builder = Builder::from_string(include_str!("ui.glade"));
    let settings = Rc::new(RefCell::new(Settings::default()));
    let session = Rc::new(RefCell::new(Session::new()));

    {
        let settings = settings.borrow();
        let folder_picker: FileChooserButton = builder.object("folder_picker").expect("Failed to init folder_picker");
        folder_picker.set_current_folder(&settings.resolve_folder());
    }

    builder.connect_signals(glib::clone!(@strong settings => move |builder, signal| {
        match signal {
            "resized" => Box::new(move |_| {
                println!("resized!");
                None
            }),
            "folder_changed" => Box::new(glib::clone!(@strong settings, @strong builder => @default-return None, move |_| {
                let folder_picker: FileChooserButton = builder.object("folder_picker").expect("Failed to init folder_picker");
                let current_folder = folder_picker.current_folder().expect("Failed to read current folder");
                let current_folder = current_folder.to_str().expect("Failed to convert curretn folder to string").to_string();
                settings.borrow_mut().folder = current_folder;
                None
            })),
            "images_changed" => Box::new(glib::clone!(@strong settings, @strong builder => @default-return None, move |_| {
                let image_options: ComboBox = builder.object("image_options").expect("Failed to init image_options");
                if let Some(index) = image_options.active() {
                    settings.borrow_mut().images_number = match index {
                        0 => 5,
                        1 => 10,
                        2 => 15,
                        3 => 20,
                        4 => 25,
                        5 => 30,
                        _ => 60,
                    };
                }
                None
            })),
            "duration_changed" => Box::new(glib::clone!(@strong settings, @strong builder => @default-return None, move |_| {
                let duration_options: ComboBox = builder.object("duration_options").expect("Failed to init duration_options");
                if let Some(index) = duration_options.active() {
                    settings.borrow_mut().duration = match index {
                        0 => None,
                        1 => Some(Duration::from_secs(15)),
                        2 => Some(Duration::from_secs(30)),
                        3 => Some(Duration::from_secs(60)),
                        4 => Some(Duration::from_secs(120)),
                        5 => Some(Duration::from_secs(300)),
                        _ => Some(Duration::from_secs(600)),
                    };
                }
                None
            })),
            "pause_changed" => Box::new(glib::clone!(@strong settings, @strong builder => @default-return None, move |_| {
                let pause_options: ComboBox = builder.object("pause_options").expect("Failed to init pause_options");
                if let Some(index) = pause_options.active() {
                    settings.borrow_mut().pause = match index {
                        0 => None,
                        1 => Some(Duration::from_secs(5)),
                        2 => Some(Duration::from_secs(10)),
                        3 => Some(Duration::from_secs(15)),
                        4 => Some(Duration::from_secs(30)),
                        _ => Some(Duration::from_secs(60)),
                    };
                }
                None
            })),
            "start_clicked" => Box::new(glib::clone!(@strong settings, @strong session, @strong builder => move |_| {
                let new_session = Session::from(&settings.borrow());

                if let Some(image) = new_session.current_image() {
                    set_image(&builder, &image);

                    session.replace(new_session);

                    set_buttons_visibility(&builder, true, true);
                    set_page(&builder, Page::Image);
                }

                None
            })),
            "back_clicked" => Box::new(glib::clone!(@strong builder => move |_| {
                set_buttons_visibility(&builder, false, false);
                set_page(&builder, Page::Settings);

                None
            })),
            "previous_image_clicked" => Box::new(glib::clone!(@strong session, @strong builder => move |_| {
                if let Some(image) = session.borrow_mut().previous_image() {
                    set_image(&builder, &image);
                }
                else {
                    set_buttons_visibility(&builder, false, false);
                    set_page(&builder, Page::Settings);
                }

                None
            })),
            "toggle_pause_clicked" => Box::new(glib::clone!(@strong session, @strong builder => move |_| {

                None
            })),
            "next_image_clicked" => Box::new(glib::clone!(@strong session, @strong builder => move |_| {
                if let Some(image) = session.borrow_mut().next_image() {
                    set_image(&builder, &image);
                }
                else {
                    set_buttons_visibility(&builder, true, false);
                    set_page(&builder, Page::Complete);
                }

                None
            })),
            _ => Box::new(|_| None),
        }
    }));

    let window: ApplicationWindow = builder.object("main_window").expect("Failed to init window");
    window.set_application(Some(application));
    window.show_all();
}

fn main() {
    let application = Application::new(
        Some("com.github.sillysyx.quick-pose"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}