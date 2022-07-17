use gtk::prelude::*;
use gtk::gdk_pixbuf::{Pixbuf, Colorspace};
use relm4::*;

use crate::{
    app::{App, AppMsg},
    session::Session,
};

pub struct SessionPage {
    pub session: Session,
    pub current_image: Pixbuf,
}

pub enum SessionPageMsg {
    NewSession(Session),
    NextImage,
    PauseImage,
    PrevImage,
}

impl Model for SessionPage {
    type Msg = SessionPageMsg;
    type Widgets = SessionPageWidgets;
    type Components = ();
}

impl ComponentUpdate<App> for SessionPage {
    fn init_model(_parent_model: &App) -> Self {
        Self {
            session: Session::new(),
            current_image: Pixbuf::new(Colorspace::Rgb, true, 8, 1, 1).unwrap(),
        }
    }

    fn update(&mut self, msg: SessionPageMsg, _components: &(), _sender: Sender<SessionPageMsg>, parent_sender: Sender<AppMsg>) {
        match msg {
            SessionPageMsg::NewSession(session) => {
                self.session = session;

                if let Some(image) = self.session.current_image() {
                    match Pixbuf::from_file(image) {
                        Ok(value) => self.current_image = value,
                        Err(_) => send!(parent_sender, AppMsg::ShowError("Failed to load image".into())),
                    };
                }
                else {
                    send!(parent_sender, AppMsg::ShowError("No images found".into()));
                }
            },
            SessionPageMsg::NextImage => {
                if let Some(image) = self.session.next_image() {
                    match Pixbuf::from_file(image) {
                        Ok(value) => self.current_image = value,
                        Err(_) => send!(parent_sender, AppMsg::ShowError("Failed to load image".into())),
                    };
                }
                else {
                    send!(parent_sender, AppMsg::ShowSessionComplete);
                }
            },
            SessionPageMsg::PauseImage => {

            },
            SessionPageMsg::PrevImage => {
                if let Some(image) = self.session.previous_image() {
                    match Pixbuf::from_file(image) {
                        Ok(value) => self.current_image = value,
                        Err(_) => send!(parent_sender, AppMsg::ShowError("Failed to load image".into())),
                    };
                }
                else {
                    send!(parent_sender, AppMsg::ShowSettings);
                }
            },
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<SessionPage, App> for SessionPageWidgets {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            // append = &gtk::ActionBar {
            //     pack_start = &gtk::Label {
            //         set_label: "1 / 10",
            //     },

            //     pack_end = &gtk::Label {
            //         set_label: "60",
            //     },
            // },

            append = &gtk::Image {
                set_vexpand: true,
                set_hexpand: true,

                set_from_pixbuf: watch!(Some(&model.current_image)),
            },
        }
    }
}