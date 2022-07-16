mod app;
mod components;
mod page;
mod session;
mod settings;

use relm4::RelmApp;

fn main() {
    let model = app::App::new();
    let app = RelmApp::new(model);
    app.run();
}