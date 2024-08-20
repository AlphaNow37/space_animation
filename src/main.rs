#![feature(portable_simd)]

use crate::app::App;

mod app;
mod logger;
mod world;
mod render_registry;
mod utils;
mod content;
mod models;
mod math;

fn main() {
    logger::init_logger();
    let mut app = App::new();
    app.run();
}
