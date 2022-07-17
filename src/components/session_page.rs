use gtk::prelude::*;
use gtk::gdk_pixbuf::{Pixbuf, Colorspace, PixbufRotation};
use relm4::*;

use crate::{
    app::{App, AppMsg},
    session::Session,
    components::*,
};

enum Timer {
    None,
    Session(usize),
    Intermission(usize),
}

pub struct SessionPageComponents {
    timer: RelmComponent<SessionTimer, SessionPage>,
}

impl Components<SessionPage> for SessionPageComponents {
    fn init_components(parent_model: &SessionPage, parent_sender: Sender<SessionPageMsg>) -> Self {
        Self {
            timer: RelmComponent::new(parent_model, parent_sender.clone()),
        }
    }

    fn connect_parent(&mut self, parent_widgets: &<SessionPage as Model>::Widgets) {
        self.timer.connect_parent(parent_widgets);
    }
}

pub struct SessionPage {
    session: Session,
    current_image: Pixbuf,
    timer: Timer,
    stopped: bool,
}

impl SessionPage {
    fn get_current_image_position(&self) -> String {
        format!("{} / {}", self.session.current_image + 1, self.session.images.len())
    }

    fn get_timer_countdown(&self) -> String {
        match self.timer {
            Timer::None => "".into(),
            Timer::Intermission(value) => value.to_string(),
            Timer::Session(value) => value.to_string(),
        }
    }

    fn intermission(&self) -> bool {
        match self.timer {
            Timer::Intermission(_) => true,
            _ => false,
        }
    }
}

pub enum SessionPageMsg {
    NewSession(Session),
    NextImage,
    PauseImage,
    PrevImage,
    Flip,
    RotateLeft,
    RotateRight,
    UpdateTimer,
}

impl Model for SessionPage {
    type Msg = SessionPageMsg;
    type Widgets = SessionPageWidgets;
    type Components = SessionPageComponents;
}

impl ComponentUpdate<App> for SessionPage {
    fn init_model(_parent_model: &App) -> Self {
        Self {
            session: Session::new(),
            current_image: Pixbuf::new(Colorspace::Rgb, true, 8, 1, 1).unwrap(),
            timer: Timer::None,
            stopped: false,
        }
    }

    fn update(&mut self, msg: SessionPageMsg, _components: &SessionPageComponents, sender: Sender<SessionPageMsg>, parent_sender: Sender<AppMsg>) {
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

                if self.session.session_time > 0 {
                    self.timer = Timer::Session(self.session.session_time);
                }
            },
            SessionPageMsg::NextImage => {
                if let Some(image) = self.session.next_image() {
                    match Pixbuf::from_file(image) {
                        Ok(value) => self.current_image = value,
                        Err(_) => send!(parent_sender, AppMsg::ShowError("Failed to load image".into())),
                    };

                    if self.session.session_time > 0 {
                        self.timer = Timer::Session(self.session.session_time);
                    }
                }
                else {
                    send!(parent_sender, AppMsg::ShowSessionComplete);
                }
            },
            SessionPageMsg::PauseImage => {
                self.stopped = !self.stopped;
            },
            SessionPageMsg::PrevImage => {
                if let Some(image) = self.session.previous_image() {
                    match Pixbuf::from_file(image) {
                        Ok(value) => self.current_image = value,
                        Err(_) => send!(parent_sender, AppMsg::ShowError("Failed to load image".into())),
                    };

                    if self.session.session_time > 0 {
                        self.timer = Timer::Session(self.session.session_time);
                    }
                }
                else {
                    send!(parent_sender, AppMsg::ShowSettings);
                }
            },
            SessionPageMsg::Flip => {
                if let Some(image) = self.current_image.flip(true) {
                    self.current_image = image;
                }
            },
            SessionPageMsg::RotateLeft => {
                if let Some(image) = self.current_image.rotate_simple(PixbufRotation::Counterclockwise) {
                    self.current_image = image;
                }
            },
            SessionPageMsg::RotateRight => {
                if let Some(image) = self.current_image.rotate_simple(PixbufRotation::Clockwise) {
                    self.current_image = image;
                }
            },
            SessionPageMsg::UpdateTimer => {
                if self.stopped {
                    return;
                }

                match self.timer {
                    Timer::Intermission(time_left) => {
                        if time_left < 2 {
                            self.timer = Timer::Session(self.session.session_time);
                            send!(sender, SessionPageMsg::NextImage);
                        }
                        else {
                            self.timer = Timer::Intermission(time_left - 1);
                        }
                    },
                    Timer::Session(time_left) => {
                        if time_left < 2 {
                            if self.session.pause_time > 0 {
                                self.timer = Timer::Intermission(self.session.pause_time);
                            }
                            else {
                                self.timer = Timer::Session(self.session.session_time);
                                send!(sender, SessionPageMsg::NextImage);
                            }
                        }
                        else {
                            self.timer = Timer::Session(time_left - 1);
                        }
                    },
                    _ => {},
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

            append = &gtk::ActionBar {
                set_visible: watch!(!model.intermission()),

                pack_start = &gtk::Label {
                    set_label: watch!(&model.get_current_image_position()),
                },

                pack_end = &gtk::Label {
                    set_label: watch!(&model.get_timer_countdown()),
                },

                set_center_widget = Some(&gtk::Box) {
                    append = &gtk::Button {
                        set_icon_name: "go-previous-symbolic",
                        set_has_frame: false,
                        
                        connect_clicked(sender) => move |_| {
                            send!(sender, SessionPageMsg::PrevImage);
                        },
                    },

                    append = &gtk::Button {
                        set_icon_name: "media-playback-start-symbolic",
                        set_has_frame: false,
                        
                        connect_clicked(sender) => move |_| {
                            send!(sender, SessionPageMsg::PauseImage);
                        },
                    },

                    append = &gtk::Button {
                        set_icon_name: "go-next-symbolic",
                        set_has_frame: false,
                        
                        connect_clicked(sender) => move |_| {
                            send!(sender, SessionPageMsg::NextImage);
                        },
                    },

                    append = &gtk::MenuButton {
                        set_icon_name: "view-more-horizontal-symbolic",
                        set_has_frame: false,

                        set_popover = Some(&gtk::Popover) {
                            set_child = Some(&gtk::Box) {
                                set_orientation: gtk::Orientation::Vertical,

                                append = &gtk::Button {
                                    set_label: "Flip",
                                    set_has_frame: false,
                                    connect_clicked(sender) => move |_| {
                                        send!(sender, SessionPageMsg::Flip);
                                    },
                                },

                                append = &gtk::Button {
                                    set_label: "Rotate left",
                                    set_has_frame: false,
                                    connect_clicked(sender) => move |_| {
                                        send!(sender, SessionPageMsg::RotateLeft);
                                    },
                                },

                                append = &gtk::Button {
                                    set_label: "Rotate right",
                                    set_has_frame: false,
                                    connect_clicked(sender) => move |_| {
                                        send!(sender, SessionPageMsg::RotateRight);
                                    },
                                },
                            }
                        }
                    },
                },
            },

            append = &gtk::Picture {
                set_visible: watch!(!model.intermission()),
                set_vexpand: true,
                set_hexpand: true,

                set_pixbuf: watch!(Some(&model.current_image)),
            },

            append = &gtk::Box {
                set_visible: watch!(model.intermission()),
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: true,
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,
                set_spacing: 50,
                
                append = &gtk::Label {
                    set_label: watch!(&model.get_timer_countdown()),
                },
    
                append = &gtk::Label {
                    set_label: "Intermission",
                },
            },
        }
    }
}