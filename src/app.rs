use gtk::prelude::*;
use relm4::*;

use crate::{
    components::*,
    page::Page,
    session::Session,
    settings::Settings,
};

pub struct AppComponents {
    settings_page: RelmComponent<SettingsPage, App>,
    session_page: RelmComponent<SessionPage, App>,
    complete_page: RelmComponent<CompletePage, App>,
    error_page: RelmComponent<ErrorPage, App>,
}

impl Components<App> for AppComponents {
    fn init_components(parent_model: &App, parent_sender: Sender<AppMsg>) -> Self {
        Self {
            settings_page: RelmComponent::new(parent_model, parent_sender.clone()),
            session_page: RelmComponent::new(parent_model, parent_sender.clone()),
            complete_page: RelmComponent::new(parent_model, parent_sender.clone()),
            error_page: RelmComponent::new(parent_model, parent_sender.clone()),
        }
    }

    fn connect_parent(&mut self, parent_widgets: &<App as Model>::Widgets) {
        self.settings_page.connect_parent(parent_widgets);
        self.session_page.connect_parent(parent_widgets);
        self.complete_page.connect_parent(parent_widgets);
        self.error_page.connect_parent(parent_widgets);
    }
}

pub enum AppMsg {
    ShowSettings,
    ShowSessionComplete,
    ShowError(String),
    StartNewSession(Settings),
}

pub struct App {
    pub page: Page,
}

impl App {
    pub fn new() -> Self {
        Self {
            page: Page::Settings,
        }
    }
}

impl Model for App {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for App {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::ShowSettings => {
                self.page = Page::Settings;
            },
            AppMsg::StartNewSession(settings) => {
                let session = Session::from(&settings);
                components.session_page.send(SessionPageMsg::NewSession(session)).unwrap();
                self.page = Page::Session;
            },
            AppMsg::ShowSessionComplete => {
                self.page = Page::Complete;
            },
            AppMsg::ShowError(error) => {
                components.error_page.send(ErrorPageMsg::ErrorMessage(error)).unwrap();
                self.page = Page::Error;
            },
        }
        true
    }
}

#[relm4::widget(pub)]
impl Widgets<App, ()> for AppWidgets {
    view! {
        main_window = gtk::ApplicationWindow {
            set_title: Some("Quick pose"),
            set_default_width: 600,
            set_default_height: 800,
            set_titlebar = Some(&gtk::HeaderBar) {
                set_show_title_buttons: true,

                pack_start = &gtk::Button {
                    set_visible: watch!(model.page != Page::Settings),
                    set_icon_name: "go-previous-symbolic",
                    set_has_frame: false,
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::ShowSettings);
                    },
                },
            },
            set_child: pages = Some(&gtk::Stack) {
                set_transition_type: gtk::StackTransitionType::SlideLeftRight,
            },
        }
    }

    fn post_init() {
        pages.add_named(components.settings_page.root_widget(), Some("settings"));
        pages.add_named(components.session_page.root_widget(), Some("session"));
        pages.add_named(components.complete_page.root_widget(), Some("complete"));
        pages.add_named(components.error_page.root_widget(), Some("error"));

        pages.set_visible_child_name("settings");
    }

    fn pre_view() {
        match model.page {
            Page::Settings => self.pages.set_visible_child_name("settings"),
            Page::Session => self.pages.set_visible_child_name("session"),
            Page::Complete => self.pages.set_visible_child_name("complete"),
            Page::Error => self.pages.set_visible_child_name("error"),
        }
    }
}