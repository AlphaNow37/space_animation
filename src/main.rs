#![feature(portable_simd, trait_upcasting)]

use crate::app::App;

mod app;
mod logger;
mod world;
mod render_registry;
mod utils;
mod content;
mod models;
mod math;
mod datastrutures;

fn main() {
    logger::init_logger();
    let mut app = App::new();
    app.run();
}
