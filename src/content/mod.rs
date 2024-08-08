use glam::Vec3;
use crate::{materials::{color::Color, materials::{FlatShape, Material}}, world::world::{EntityRef, World}};

pub fn build(world: &mut World) {
    let mut view = world.view(EntityRef::ROOT);
    let n = 100;
    let step = 2. / n as f32;
    for x in 0..n {
        let x = x as f32 * step - 1.;
        for y in 0..n {
            let y = y as f32 * step - 1.;
            view.new_child()
                .with_material(Material::UniformFlat { col: Color::DEBUG, shape: FlatShape::Triangle(
                    [Vec3::new(x, y, 0.), Vec3::new(x+step, y, 0.), Vec3::new(x, y+step, 0.)]
                ) })
                .build();
        }
    }
}
