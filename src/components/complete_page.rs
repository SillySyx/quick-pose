use gtk::prelude::*;
use relm4::*;

use crate::app::{App, AppMsg};

pub struct CompletePage {
}

pub enum CompletePageMsg {
}

impl Model for CompletePage {
    type Msg = CompletePageMsg;
    type Widgets = CompletePageWidgets;
    type Components = ();
}

impl ComponentUpdate<App> for CompletePage {
    fn init_model(_parent_model: &App) -> Self {
        Self {

        }
    }

    fn update(&mut self, msg: CompletePageMsg, _components: &(), _sender: Sender<CompletePageMsg>, _parent_sender: Sender<AppMsg>) {
        match msg {
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<CompletePage, App> for CompletePageWidgets {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,
            set_spacing: 50,
            
            append = &gtk::Image {
                set_icon_name: Some("selection-mode-symbolic"),
                set_pixel_size: 100,
            },

            append = &gtk::Label {
                set_label: "Session complete",
            },
        }
    }
}