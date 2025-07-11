#![feature(portable_simd)]

use app::App;

pub mod app;
pub mod datastrutures;
pub mod logger;
pub mod math;
pub mod render_registry;
pub mod settings;
pub mod utils;
pub mod world;


pub fn run(build_fun: impl FnMut() -> world::world_builder::WorldsBuilder + 'static) {
    logger::init_logger();
    let mut app = App::new(build_fun);
    app.run();
}
