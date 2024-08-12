use glam::Vec3A;
use crate::world::{color::Color, combinators::Interpolate, material::UniformTri, pack::Pack3, shape::Triangle, variator::Variator, world::World};

pub fn build(world: &mut World) {
    let a = world.push(Interpolate(0., 1.).time_mod(1.).time_mul(0.2));
    world.push_mat(
        UniformTri {
            shape: Triangle(Vec3A::new(0., 0., 0.), Vec3A::new(0., 1., 0.), Vec3A::new(1., 0., 1.)),
            color: Color::WHITE,
        }
    );
    world.push_mat(
        UniformTri {
            shape: Triangle(
                Vec3A::new(1., 1., 0.), 
                Vec3A::new(0., 0., 0.0), 
                Pack3(0., 1., a)
            ),
            color: Interpolate(Color::DEBUG, Color::BLACK).time_mod(1.),
        }
    );
    // let mut view = world.view(EntityRef::ROOT);
    // // let n = 100;
    // // let step = 2. / n as f32;
    // // for x in 0..n {
    // //     let x = x as f32 * step - 1.;
    // //     for y in 0..n {
    // //         let y = y as f32 * step - 1.;
    // //         view.new_child()
    // //             .with_material(Material::UniformFlat { col: Color::DEBUG, shape: FlatShape::Triangle(
    // //                 [Vec3::new(x, y, 0.), Vec3::new(x+step, y, 0.), Vec3::new(x, y+step, 0.)]
    // //             ) })
    // //             .build();
    // //     }
    // // }
    // view.new_child()
    //     .with_material(Material::UniformFlat { col: Color::DEBUG, shape: FlatShape::Triangle(
    //         [Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.5), Vec3::new(0., 1., 0.5)]
    //     ) })
    //     .build();
    // view.new_child()
    //     .with_material(Material::UniformFlat { col: Color::WHITE, shape: FlatShape::Triangle(
    //         [Vec3::new(0., 0., 0.5), Vec3::new(1., 1., 0.), Vec3::new(0., 1., 2.)]
    //     ) })
    //     .build();
}
