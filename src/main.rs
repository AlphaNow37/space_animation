use crate::app::App;

mod app;
mod logger;
mod world;
mod materials;
mod utils;
mod content;

fn main() {
    logger::init_logger();
    let mut app = App::new();
    app.run();
}
