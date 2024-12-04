use crate::{models::put_axis, world::{primitives::color::Color, world::World}};
use crate::math::{Polynomial, rotate_x, scale, ToAngle, trans, Transform, vec3};
use crate::world::variators::combinators::Interpolate;
use crate::world::variators::variator::{UpdateCtx, Variator};
use crate::world::visuals::{Cube, Sphere, Sponge};

fn put_cube(world: &mut World, x: usize, y: usize) {
    let pos = world.push(move |_ctx, _world: &World| {
        trans(x as f32 * 2., 5., y as f32 * 2.,) * rotate_x(45.0f32.deg())
    });
    if (x+y) % 2 == 0 {
        let col = world.push(Color::WHITE);
        world.push_visual(col);
    } else {
        let cols = world.push((Color::RED, Color::WHITE));
        world.push_visual(Sponge(cols));
    }

    let loc = world.push(scale(0.5, 0.5, 0.5));
    world.push_visual((
        pos,
        Cube(loc),
    ));
}
  
pub fn build(world: &mut World) {
    put_axis(world, Transform::ID);

    for x in 0..5 {
        for y in 0..5 {
            put_cube(world, x, y);
        }
    }

    // let a = world.push(Interpolate(0., 1.).time_mod(1.).time_mul(0.2));

    // put_axis(world, Affine3A::IDENTITY);

    // put_axis(world, Interpolate(
    //     Transform::ID,
    //     Transform::from_cols(Vec3::X, Vec3::Z, -Vec3::Y).with_trans(trans(0., 1., 0.)),
    // ).time_lea(0.5, 0.5).time_sin(20.));

    put_axis(world, trans(5., 5., 5.));

    // world.push_mat(UniformTri {
    //     color: Color::RED,
    //     shape: Triangle(Vec3A::X, Vec3A::Y, Vec3A::Z),
    // });

    // world.push_mat(UniformTri {
    //     shape: Cube(trans(2., -2., 2.)*rotate_x(180.0f32.deg())),
    //     color: Color::RED,
    //     global: Transform::ID,
    // });

    // world.push_mat(
    //     SpongeTri {
    //         global: trans(-2., 4., -2.),
    //         shape: Cube(|_ctx: UpdateCtx, _world: &World| scale(0.5, 0.5, 0.5)),
    //         color1: Color::RED,
    //         color2: Color::WHITE,
    //     },
    // );

    let surf1 = world.push(|ctx: UpdateCtx, world: &World| Polynomial::new_bezier_surface([
        [vec3(0., 0., 3.), vec3(1., 2., 0.), vec3(2., 2., -2.)],
        [vec3(0., -1., 2.), vec3(2., 1., 2.), vec3(2., 3., 0.)],
        [vec3(1., 0., 2.), vec3(2., 0., 2.), vec3(3., 2., ctx.time.sin())],
    ]).to_size::<4, 4>());
    let surf2 = world.push(|ctx: UpdateCtx, world: &World| Polynomial::new_bezier_surface([
        [vec3(4., ctx.time.sin(), 1.), vec3(3., 3., 0.), vec3(2., 2., -2.)],
        [vec3(2., -1., 2.), vec3(2., 1., 2.), vec3(2., 3., 0.)],
        [vec3(1., 0., 2.), vec3(0., 0., 3.), vec3(1., 1., 2.)],
    ]).to_size::<4, 4>());
    let surf = world.push(Interpolate(surf1, surf2).time_sin(4.));
    let tr = world.push(trans(-5., 0., 0.));
    world.push_visual((
        tr,
        surf,
    ));

    // world.push_mat(
    //     UniformTri {
    //         shape: Pyramid(
    //             Triangle(vec3(-5., 5., -5.), vec3(-4., 5., -5.), vec3(-5., 4., -6.)),
    //             vec3(-5., 6., -5.)
    //         ),
    //         color: Color::DEBUG,
    //         global: Transform::ID,
    //     },
    // );

    let loc = world.push((|ctx: UpdateCtx, _: &World| scale(ctx.time+2., ctx.time+2., ctx.time+2.)).time_sin(4.));
    let glob = world.push(trans(-3., -3., -3.));
    let col = world.push(Color::DEBUG);

    world.push_visual((glob, col, Sphere(loc)));
}
// from_rotation_y(180.0f32.to_radians())
