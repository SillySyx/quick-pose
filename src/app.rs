use gtk::prelude::*;
use relm4::*;

use crate::{
    components::*,
    page::Page,
    session::Session,
};

pub struct AppComponents {
    settings_page: RelmComponent<SettingsPage, App>,
    session_page: RelmComponent<SessionPage, App>,
    complete_page: RelmComponent<SessionPage, App>,
    error_page: RelmComponent<SessionPage, App>,
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
    ShowError(String),
    StartNewSession,
}

pub struct App {
    pub page: Page,
    pub error: Option<String>,
    pub session: Session,
}

impl App {
    pub fn new() -> Self {
        Self {
            page: Page::Settings,
            error: None,
            session: Session::new(),
        }
    }
}

impl Model for App {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for App {
    fn update(&mut self, msg: AppMsg, _components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::ShowSettings => {
                self.page = Page::Settings;
            },
            AppMsg::StartNewSession => {
                self.page = Page::Session;
            },
            AppMsg::ShowError(error) => {
                self.error = Some(error);
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
                    set_visible: watch!(model.page == Page::Session),
                    set_icon_name: "go-previous-symbolic",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::ShowSettings);
                    },
                },

                pack_end = &gtk::MenuButton {
                    set_visible: watch!(model.page == Page::Session),
                    set_icon_name: "view-more-horizontal-symbolic",

                    set_popover = Some(&gtk::Popover) {
                        set_child = Some(&gtk::Box) {
                            append = &gtk::Button {
                                set_label: "hello",
                            },
                        }
                    }
                },

                pack_end = &gtk::Button {
                    set_visible: watch!(model.page == Page::Session),
                    set_icon_name: "go-next-symbolic",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::ShowSettings);
                    },
                },

                pack_end = &gtk::Button {
                    set_visible: watch!(model.page == Page::Session),
                    set_icon_name: "media-playback-start-symbolic",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::ShowSettings);
                    },
                },

                pack_end = &gtk::Button {
                    set_visible: watch!(model.page == Page::Session),
                    set_icon_name: "go-previous-symbolic",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::ShowSettings);
                    },
                },
            },
            set_child: pages = Some(&gtk::Stack) {
                set_transition_type: gtk::StackTransitionType::SlideLeftRight,

                add_child: session = &gtk::Box {
                },

                add_child: complete = &gtk::Box {
                    append = &gtk::Label {
                        set_label: "complete",
                    },
                },

                add_child: error = &gtk::Box {
                    append = &gtk::Label {
                        set_label: "error",
                    },
                },
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