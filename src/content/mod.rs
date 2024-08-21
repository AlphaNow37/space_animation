use crate::{models::put_axis, world::{primitives::color::Color, world::World}};
use crate::math::{rotate_x, scale, ToAngle, trans, Transform, vec3};
use crate::world::variators::variator::UpdateCtx;
use crate::world::visuals::material::{SpongeTri, UniformSphere, UniformTri};
use crate::world::visuals::shape::{Cube, Pyramid, Triangle};

fn put_cube(world: &mut World) {
    let pos = world.push(|_ctx, _world: &World| {
        trans(0., 5., 0.) * rotate_x(45.0f32.deg())
    });
    world.push_mat(UniformTri {
        shape: Cube(Transform::ID),
        color: Color::WHITE,
        global: pos,
    })
}

pub fn build(world: &mut World) {
    put_axis(world, Transform::ID);

    put_cube(world);

    // let a = world.push(Interpolate(0., 1.).time_mod(1.).time_mul(0.2));

    // put_axis(world, Affine3A::IDENTITY);

    // put_axis(world, Interpolate(
    //     Transform::ID,
    //     Transform::from_cols(Vec3::X, Vec3::Z, -Vec3::Y).with_trans(trans(0., 1., 0.)),
    // ).time_lea(0.5, 0.5).time_sin(0.05));

    put_axis(world, trans(5., 5., 5.));

    // world.push_mat(UniformTri {
    //     color: Color::RED,
    //     shape: Triangle(Vec3A::X, Vec3A::Y, Vec3A::Z),
    // });

    world.push_mat(UniformTri {
        shape: Cube(trans(2., -2., 2.)*rotate_x(180.0f32.deg())),
        color: Color::RED,
        global: Transform::ID,
    });

    world.push_mat(
        SpongeTri {
            global: trans(-2., 4., -2.),
            shape: Cube(|_ctx: UpdateCtx, _world: &World| scale(0.5, 0.5, 0.5)),
            color1: Color::RED,
            color2: Color::WHITE,
        },
    );

    world.push_mat(
        UniformTri {
            shape: Pyramid(
                Triangle(vec3(-5., 5., -5.), vec3(-4., 5., -5.), vec3(-5., 4., -6.)),
                vec3(-5., 6., -5.)
            ),
            color: Color::DEBUG,
            global: Transform::ID,
        },
    );

    world.push_mat(
        UniformSphere {
            local: scale(1., 2., 1.),
            color: Color::DEBUG,
            global: trans(3., 3., 3.),
        }
    )
}
// from_rotation_y(180.0f32.to_radians())
