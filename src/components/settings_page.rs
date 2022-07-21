use gtk::prelude::*;
use relm4::*;

use std::time::Duration;
use super::folder_picker::{FolderPicker, FolderPickerMsg};

use crate::{
    settings::Settings,
    app::{App, AppMsg},
};

pub struct SettingsPage {
    settings: Settings,
}

impl SettingsPage {
    pub fn get_folder_text(&self) -> String {
        if self.settings.folder.is_empty() {
            return "None".into();
        }

        self.settings.folder.clone()
    }
}

pub enum SettingsPageMsg {
    Start,
    SelectFolder,
    FolderChanged(String),
    ImagesChanged(usize),
    DurationChanged(Option<u64>),
    PauseChanged(Option<u64>),
}

pub struct SettingsPageComponents {
    folder_picker: RelmComponent<FolderPicker, SettingsPage>,
}

impl Components<SettingsPage> for SettingsPageComponents {
    fn init_components(parent_model: &SettingsPage, parent_sender: Sender<SettingsPageMsg>) -> Self {
        Self {
            folder_picker: RelmComponent::new(parent_model, parent_sender.clone()),
        }
    }

    fn connect_parent(&mut self, parent_widgets: &<SettingsPage as Model>::Widgets) {
        self.folder_picker.connect_parent(parent_widgets);
    }
}

impl Model for SettingsPage {
    type Msg = SettingsPageMsg;
    type Widgets = SettingsPageWidgets;
    type Components = SettingsPageComponents;
}

impl ComponentUpdate<App> for SettingsPage {
    fn init_model(_parent_model: &App) -> Self {
        Self {
            settings: Settings::new(),
        }
    }

    fn update(&mut self, msg: SettingsPageMsg, components: &SettingsPageComponents, _sender: Sender<SettingsPageMsg>, parent_sender: Sender<AppMsg>) {
        match msg {
            SettingsPageMsg::Start => {
                send!(parent_sender, AppMsg::StartNewSession(self.settings.clone()));
            },
            SettingsPageMsg::SelectFolder => {
                components.folder_picker.send(FolderPickerMsg::Show).unwrap();
            },
            SettingsPageMsg::FolderChanged(value) => {
                self.settings.folder = value;    
            },
            SettingsPageMsg::ImagesChanged(value) => {
                self.settings.images_number = value;    
            },
            SettingsPageMsg::DurationChanged(value) => {
                self.settings.duration = match value {
                    Some(value) => Some(Duration::from_secs(value)),
                    None => None,
                };
            },
            SettingsPageMsg::PauseChanged(value) => {
                self.settings.pause = match value {
                    Some(value) => Some(Duration::from_secs(value)),
                    None => None,
                };
            },
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<SettingsPage, App> for SettingsPageWidgets {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 50,
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,
            set_width_request: 350,
            set_margin_top: 30,
            set_margin_bottom: 30,
            set_margin_start: 30,
            set_margin_end: 30,

            append = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,

                append = &gtk::Box {
                    set_spacing: 5,
                    set_homogeneous: true,

                    append = &gtk::Label {
                        set_halign: gtk::Align::Start,
                        set_label: "Select folder",
                    },
                    append = &gtk::Image {
                        set_halign: gtk::Align::End,
                        set_icon_name: Some("help-about-symbolic"),
                        set_has_tooltip: true,
                        set_tooltip_text: Some("Images will be randomly selected from this folder"),
                    },
                },
                
                append = &gtk::Button {
                    set_icon_name: "go-down-symbolic",

                    set_child = Some(&gtk::Label) {
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        set_max_width_chars: 5,
                        set_label: watch!(&model.get_folder_text()),
                    },

                    connect_clicked(sender) => move |_| {
                        send!(sender, SettingsPageMsg::SelectFolder);
                    },
                },
            },
            append = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                
                append = &gtk::Box {
                    set_spacing: 5,
                    set_homogeneous: true,

                    append = &gtk::Label {
                        set_halign: gtk::Align::Start,
                        set_label: "Images",
                    },
                    append = &gtk::Image {
                        set_halign: gtk::Align::End,
                        set_icon_name: Some("help-about-symbolic"),
                        set_has_tooltip: true,
                        set_tooltip_text: Some("Select how many images that should be randomly selected from the folder"),
                    },
                },

                append: images = &gtk::ComboBoxText {
                    connect_changed(sender) => move |combo_box| {
                        if let Some(id) = combo_box.active_id() {
                            let value = id.parse::<usize>().unwrap_or(5);
                            send!(sender, SettingsPageMsg::ImagesChanged(value));
                        }
                    },
                },
            },
            append = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                
                append = &gtk::Box {
                    set_spacing: 5,
                    set_homogeneous: true,

                    append = &gtk::Label {
                        set_halign: gtk::Align::Start,
                        set_label: "Duration",
                    },
                    append = &gtk::Image {
                        set_halign: gtk::Align::End,
                        set_icon_name: Some("help-about-symbolic"),
                        set_has_tooltip: true,
                        set_tooltip_text: Some("Limit how long each image is displayed before switching to the next one"),
                    },
                },
                append: duration = &gtk::ComboBoxText {
                    connect_changed(sender) => move |combo_box| {
                        if let Some(id) = combo_box.active_id() {
                            let value = match id.parse::<u64>() {
                                Ok(value) => Some(value),
                                Err(_) => None,
                            };
                            send!(sender, SettingsPageMsg::DurationChanged(value));
                        }
                    },
                },
            },
            append = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                
                append = &gtk::Box {
                    set_spacing: 5,
                    set_homogeneous: true,

                    append = &gtk::Label {
                        set_halign: gtk::Align::Start,
                        set_label: "Intermission",
                    },
                    append = &gtk::Image {
                        set_halign: gtk::Align::End,
                        set_icon_name: Some("help-about-symbolic"),
                        set_has_tooltip: true,
                        set_tooltip_text: Some("Add extra time between each image, useful if you want to perform some smaller tasks after each image"),
                    },
                },
                append: pause = &gtk::ComboBoxText {
                    connect_changed(sender) => move |combo_box| {
                        if let Some(id) = combo_box.active_id() {
                            let value = match id.parse::<u64>() {
                                Ok(value) => Some(value),
                                Err(_) => None,
                            };
                            send!(sender, SettingsPageMsg::PauseChanged(value));
                        }
                    },
                },
            },
            append = &gtk::Button {
                set_label: "Start",
                connect_clicked(sender) => move |_| {
                    send!(sender, SettingsPageMsg::Start);
                },
            },
        }
    }

    fn post_init() {
        images.append(Some("5"), "5");
        images.append(Some("10"), "10");
        images.append(Some("15"), "15");
        images.append(Some("30"), "30");
        images.append(Some("60"), "60");
        images.set_active_id(Some("5"));

        duration.append(Some("0"), "None");
        duration.append(Some("15"), "15 seconds");
        duration.append(Some("30"), "30 seconds");
        duration.append(Some("60"), "1 minute");
        duration.append(Some("120"), "2 minutes");
        duration.append(Some("300"), "5 minutes");
        duration.append(Some("600"), "10 minutes");
        duration.set_active_id(Some("60"));

        pause.append(Some("0"), "None");
        pause.append(Some("5"), "5 seconds");
        pause.append(Some("10"), "10 seconds");
        pause.append(Some("15"), "15 seconds");
        pause.append(Some("30"), "30 seconds");
        pause.append(Some("60"), "1 minute");
        pause.set_active_id(Some("0"));
    }
}