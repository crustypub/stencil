mod app;
mod configmanager;
mod models;
mod utils;

fn main() {
    let mut app = app::App::new();
    app.run();
}
