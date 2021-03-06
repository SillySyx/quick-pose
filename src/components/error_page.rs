use gtk::prelude::*;
use relm4::*;

use crate::app::{App, AppMsg};

pub struct ErrorPage {
    error: String,
}

pub enum ErrorPageMsg {
    ErrorMessage(String),
}

impl Model for ErrorPage {
    type Msg = ErrorPageMsg;
    type Widgets = ErrorPageWidgets;
    type Components = ();
}

impl ComponentUpdate<App> for ErrorPage {
    fn init_model(_parent_model: &App) -> Self {
        Self {
            error: "".into(),
        }
    }

    fn update(&mut self, msg: ErrorPageMsg, _components: &(), _sender: Sender<ErrorPageMsg>, _parent_sender: Sender<AppMsg>) {
        match msg {
            ErrorPageMsg::ErrorMessage(error) => {
                self.error = error;
            }
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<ErrorPage, App> for ErrorPageWidgets {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,
            
            append = &gtk::Label {
                set_label: watch!(&model.error),
            },
        }
    }
}