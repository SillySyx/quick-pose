use gtk::prelude::*;
use relm4::*;

use super::{SettingsPage, SettingsPageMsg};

pub struct FolderPicker {
    visible: bool,
}

pub enum FolderPickerMsg {
    Show,
    Close,
    SelectFolder(String),
}

impl ComponentUpdate<SettingsPage> for FolderPicker {
    fn init_model(_parent_model: &SettingsPage) -> Self {
        Self { 
            visible: false,
        }
    }

    fn update(&mut self, msg: FolderPickerMsg, _components: &(), _sender: Sender<FolderPickerMsg>, parent_sender: Sender<SettingsPageMsg>) {
        match msg {
            FolderPickerMsg::Show => {
                self.visible = true;
            },
            FolderPickerMsg::Close => {
                self.visible = false;
            },
            FolderPickerMsg::SelectFolder(folder) => {
                self.visible = false;
                send!(parent_sender, SettingsPageMsg::FolderChanged(folder));
            }
        }
    }
}

impl Model for FolderPicker {
    type Msg = FolderPickerMsg;
    type Widgets = FolderPickerWidgets;
    type Components = ();
}

#[relm4::widget(pub)]
impl Widgets<FolderPicker, SettingsPage> for FolderPickerWidgets {
    view! {
        gtk::FileChooserDialog {
            set_modal: true,
            set_action: gtk::FileChooserAction::SelectFolder,
            set_visible: watch!(model.visible),

            add_button: args!("Select", gtk::ResponseType::Accept),

            connect_response(sender) => move |dialog, response| {
                if response == gtk::ResponseType::Accept {
                    if let Some(folder) = dialog.current_folder() {
                        let folder = folder.path().unwrap();
                        let folder = folder.to_str().unwrap();
                        send!(sender, FolderPickerMsg::SelectFolder(folder.into()));
                    }
                }

                send!(sender, FolderPickerMsg::Close);
            }
        }
    }
}