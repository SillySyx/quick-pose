use gtk::prelude::*;
use relm4::*;

use crate::app::{App, AppMsg};

pub struct SessionPage {

}

pub enum SessionPageMsg {
}

impl Model for SessionPage {
    type Msg = SessionPageMsg;
    type Widgets = SessionPageWidgets;
    type Components = ();
}

impl ComponentUpdate<App> for SessionPage {
    fn init_model(_parent_model: &App) -> Self {
        Self {

        }
    }

    fn update(&mut self, msg: SessionPageMsg, _components: &(), _sender: Sender<SessionPageMsg>, _parent_sender: Sender<AppMsg>) {
        match msg {
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<SessionPage, App> for SessionPageWidgets {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,
            
            append = &gtk::Label {
                set_label: "session",
            },
        }
    }
}