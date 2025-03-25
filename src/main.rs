#![feature(portable_simd, trait_upcasting)]

use crate::app::App;

mod app;
mod content;
mod datastrutures;
mod logger;
mod math;
mod models;
mod render_registry;
mod utils;
mod world;
mod settings;

fn main() {
    logger::init_logger();
    let mut app = App::new();
    app.run();
}
