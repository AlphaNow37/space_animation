use crate::{math::rotate_z, models::put_axis, world::{primitives::color::Color, visuals::{Tiled, Triangle}, world::World}};
use crate::math::{Polynomial, rotate_x, scale, ToAngle, trans, Transform, vec3};
use crate::world::variators::combinators::{FloatExt, Interpolate};
use crate::world::variators::variator::Variator;
use crate::world::visuals::{Cube, Sphere, Sponge};

fn put_cube(world: &mut World, x: usize, y: usize) {
    let pos = world.push(move |_world: &World| {
        trans(x as f32 * 2., 5., y as f32 * 2.,) * rotate_x(45.0f32.deg())
    });
    if (x+y) % 2 == 0 {
        let col = world.push(Color::WHITE);
        world.push_visual(col);
    } else {
        let cols = world.push((Color::RED, Color::WHITE));
        world.push_visual(Sponge(cols));
    }
    let t = world.push(Transform::from_transf(0.5, 0.5, 1.5));// * rotate_x(45.0.deg()));
    world.push_visual((t, Cube(pos)));
}

pub fn build(world: &mut World) {
    let time = |w: &World| w.settings.base_time;

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

    let surf1 = world.push(move |world: &World| Polynomial::new_bezier_surface([
        [vec3(0., 0., 3.), vec3(1., 2., 0.), vec3(2., 2., -2.)],
        [vec3(0., -1., 2.), vec3(2., 1., 2.), vec3(2., 3., 0.)],
        [vec3(1., 0., 2.), vec3(2., 0., 2.), vec3(3., 2., time.update(world).sin())],
    ]).to_size::<4, 4>());
    let surf2 = world.push(move |world: &World| Polynomial::new_bezier_surface([
        [vec3(4., time.update(world).sin(), 1.), vec3(3., 3., 0.), vec3(2., 2., -2.)],
        [vec3(2., -1., 2.), vec3(2., 1., 2.), vec3(2., 3., 0.)],
        [vec3(1., 0., 2.), vec3(0., 0., 3.), vec3(1., 1., 2.)],
    ]).to_size::<4, 4>());
    let surf = world.push(Interpolate(surf1, surf2, time.sin(2., 1.)));
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
    let time_sin = time.sin(4., 1.);
    let loc = world.push(move |world: &World| scale(time_sin.update(world)+2., time_sin.update(world)+2., time_sin.update(world)+2.));
    let glob = world.push(trans(-3., -3., -3.));
    let col = world.push(Color::DEBUG);

    world.push_visual((glob, col, Sphere(loc)));

    let refs = world.push_multi(|world: &World| [0., 1., 2.]);

    let cols = world.push((Color::RED, Color::WHITE));
    let mat = world.push_visual(Sponge(cols));
    let a = world.push(vec3(0., 0., 1.));
    let b = world.push(vec3(1., 0., 0.));
    let c = world.push(vec3(0., 1., 0.));
    let tile_plane = world.push(move |world: &World| rotate_z(time.update(world).rad()));
    // let tile_plane = world.push(Transform::ID);
    let glob = world.push(trans(0., 0., 2.));
    world.push_visual((glob, mat, Tiled(Triangle(a, b, c), tile_plane)))
}
