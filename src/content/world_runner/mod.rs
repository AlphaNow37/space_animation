use std::f32::consts::TAU;

use rand::{Rng, rng};

use crate::{
    datastrutures::{
        graph::Graph,
        sampler_linker::{DimensionParam, SampleLinkPointParam},
    },
    math::{Transform, Vec3, vec3},
    utils::{Length, VectorSpace, Zero},
    world::{
        primitives::color::Color,
        visuals::{Pipe, Sphere},
        world::World,
    },
};

pub fn build(world: &mut World) {
    let mut rng = rng();

    let id = world.push(Transform::ID);
    // let col = world.push(Color::RED);
    // world.push_visual((id, col, Sphere(id)));

    let (graph, points) = SampleLinkPointParam {
        dims: [
            DimensionParam {
                a: Vec3::ZERO,
                b: Vec3::X * 20.,
                mean_variation: 0.0,
                point_amount: 20,
            },
            DimensionParam {
                a: Vec3::ZERO,
                b: Vec3::Y * 20.,
                mean_variation: 0.,
                point_amount: 20,
            },
            DimensionParam {
                a: Vec3::ZERO,
                b: Vec3::Z * 20.,
                mean_variation: 0.0,
                point_amount: 20,
            },
        ],
    }
    .eval(&mut rng);
    for p in &points {
        let pos = world.push(Transform::from_transv(*p).scaled(vec3(0.25, 0.25, 0.25)));
        let col = world.push(Color::from_oklchf(0.5, 0.2, rng.random_range(0.0..TAU)));
        world.push_visual((id, col, Sphere(pos)));
    }
    for i in 0..points.len() {
        for i2 in graph.iter_neighboors(i) {
            if i2 <= i {
                continue;
            }
            let p1 = points[i];
            let p2 = points[i2];
            let tr = Transform::from_transv(p1.mid(p2))
                * Transform::from_z_looking_at(p2 - p1).scaled(vec3(
                    0.1,
                    0.1,
                    (p2 - p1).length() / 2.,
                ));
            let pos = world.push(tr);
            let col = world.push(Color::from_oklchf(0.5, 0.2, rng.random_range(0.0..TAU)));
            world.push_visual((id, col, Pipe(pos)));
        }
    }
}
