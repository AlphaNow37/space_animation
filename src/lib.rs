#![feature(portable_simd)]

use app::App;

mod app;
pub mod content;
mod datastrutures;
mod logger;
mod math;
mod models;
mod render_registry;
mod settings;
mod utils;
mod world;


pub fn run(build_fun: impl FnMut() -> world::world_builder::WorldsBuilder + 'static) {
    logger::init_logger();
    let mut app = App::new(build_fun);
    app.run();
}
