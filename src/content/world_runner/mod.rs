use std::f32::consts::PI;

use rand::{Rng, rng};

use lib_space_animation::world::world::Worlds;
use lib_space_animation::{
    datastrutures::{
        graph::Graph,
        sampler_linker::{DimensionParam, SampleLinkPointParam},
    },
    math::{Dir, Transform, Vec3, vec3},
    utils::{Length, Zero},
    world::{
        primitives::color::Color,
        variators::{references::Ref, variator::Variator},
        visuals::{Pipe, Sphere},
        world_builder::WorldsBuilder,
    },
};

pub fn build() -> WorldsBuilder {
    let worlds = WorldsBuilder::default();
    let mut world = worlds.add_world(5);

    let mut rng = rng();

    let id = world.push(Transform::ID);
    // let col = world.push(Color::RED);
    // world.push_visual((id, col, Sphere(id)));

    let (graph, points) = SampleLinkPointParam {
        dims: [
            DimensionParam {
                a: Vec3::ZERO,
                b: Vec3::X * 15.,
                mean_variation: 0.5,
                point_amount: 10,
            },
            DimensionParam {
                a: Vec3::ZERO,
                b: Vec3::Y * 15.,
                mean_variation: 0.5,
                point_amount: 10,
            },
            DimensionParam {
                a: Vec3::ZERO,
                b: Vec3::Z * 15.,
                mean_variation: 0.5,
                point_amount: 10,
            },
        ],
    }
    .eval(&mut rng);

    let local_sphere = world.push(Transform::from_scalef(0.2, 0.2, 0.2));

    let transforms = points
        .iter()
        .copied()
        .map(|pos: Vec3| {
            let a = *rng.random::<Dir>() * 0.3;
            let b = *rng.random::<Dir>() * 0.3;
            world.push(move |worlds: &Worlds| {
                Transform::from_transv(
                    pos + a * worlds.settings.base_time.sin() + b * worlds.settings.base_time.cos(),
                )
            })
        })
        .collect::<Vec<_>>();

    for (i, _) in points.iter().enumerate() {
        let col: Ref<Color> = world.push(Color::from_oklchf(0.5, 0.3, rng.random_range(-PI..PI)));
        world.push_visual((transforms[i], col, Sphere(local_sphere)));
    }
    for i in 0..points.len() {
        for i2 in graph.iter_neighboors(i) {
            if i2 <= i {
                continue;
            }
            let p1 = transforms[i];
            let p2 = transforms[i2];
            let tr = world.push(move |worlds: &Worlds| {
                let p1pos = p1.update(worlds).trans();
                let p2pos = p2.update(worlds).trans();
                Transform::from_transv(p1pos)
                    * Transform::from_z_looking_at(p2pos - p1pos).scaled(vec3(
                        0.05,
                        0.05,
                        (p2pos - p1pos).length(),
                    ))
            });
            let col = world.push(Color::from_oklchf(0.2, 0.1, rng.random_range(-PI..PI)));
            world.push_visual((id, col, Pipe(tr)));
        }
    }

    let worlds = world.finalize();

    let mut world = worlds.add_world(1);
    let col = world.push(Color::RED);
    let tr = world.push(Transform::ID * 2.);
    world.push_visual((id, col, Sphere(tr)));
    world.set_bounding_box(tr);
    let worlds = world.finalize();

    worlds
}
