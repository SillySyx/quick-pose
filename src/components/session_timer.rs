use relm4::*;
use std::{thread, time::Duration};

use crate::components::{SessionPage, SessionPageMsg};

pub struct SessionTimer {
}

pub enum SessionTimerMsg {
    Tick,
}

impl Model for SessionTimer {
    type Msg = SessionTimerMsg;
    type Widgets = SessionTimerWidgets;
    type Components = ();
}

impl ComponentUpdate<SessionPage> for SessionTimer {
    fn init_model(_parent_model: &SessionPage) -> Self {
        Self {
        }
    }

    fn update(&mut self, msg: SessionTimerMsg, _components: &(), _sender: Sender<SessionTimerMsg>, parent_sender: Sender<SessionPageMsg>) {
        match msg {
            SessionTimerMsg::Tick => {
                send!(parent_sender, SessionPageMsg::UpdateTimer);
            },
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<SessionTimer, SessionPage> for SessionTimerWidgets {
    view! {
        &gtk::Box {
        }
    }

    fn post_init() {
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                send!(sender, SessionTimerMsg::Tick);
            }
        });
    }
}