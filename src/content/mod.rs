use glam::{Affine3A, Mat3, Mat3A, Vec3, Vec3A};
use crate::{models::put_axis, world::{primitives::color::Color, point::WithRotation, variators::variator::Variator, world::World}};
use crate::world::primitives::camera::TrackCamera;
use crate::world::primitives::angle::Angle;
use crate::world::variators::combinators::Interpolate;
use crate::world::visuals::material::UniformTri;
use crate::world::visuals::shape::{Cube, Pyramid, Triangle};

fn put_cube(world: &mut World) {
    let pos = world.push(|_ctx, _world: &World| {
        Affine3A::from_translation(Vec3::new(0., 5., 0.))
            * Affine3A::from_rotation_x(45.0f32.to_radians())
    });
    world.push_mat(UniformTri {
        shape: Cube(pos),
        color: Color::WHITE,
    })
}

pub fn build(world: &mut World) {
    put_axis(world, Affine3A::IDENTITY);

    put_cube(world);

    // let a = world.push(Interpolate(0., 1.).time_mod(1.).time_mul(0.2));

    // put_axis(world, Affine3A::IDENTITY);

    put_axis(world, Interpolate(
        Affine3A::IDENTITY,
        Affine3A::from_mat3_translation(Mat3::from_cols(Vec3::X, Vec3::Z, -Vec3::Y), Vec3::Y),
    ).time_lea(0.5, 0.5).time_sin(0.05));

    put_axis(world, Affine3A::from_translation(Vec3::new(5., 5., 5.)));

    let pos = Interpolate(
        Vec3A::new(0.5, 0.5, 0.),
        Vec3A::new(0.5, 0.5, 10.),
    );
    let pos = WithRotation(pos, Mat3A::from_cols(-Vec3A::X, -Vec3A::Y, -Vec3A::Z)).time_mod(1.).time_mul(0.1);
    world.push(TrackCamera(pos, Angle::from_deg(90.)));

    // world.push_mat(UniformTri {
    //     color: Color::RED,
    //     shape: Triangle(Vec3A::X, Vec3A::Y, Vec3A::Z),
    // });

    world.push_mat(UniformTri {
        shape: Cube(Affine3A::from_translation(Vec3::new(2., -2., 2.))*Affine3A::from_rotation_x(180.0f32.to_radians())),
        color: Color::RED,
    });

    world.push_mat(
        UniformTri {
            shape: Pyramid(
                Triangle(Vec3A::new(-5., 5., -5.), Vec3A::new(-4., 5., -5.), Vec3A::new(-5., 4., -6.)),    
                Vec3A::new(-5., 6., -5.)
            ),
            color: Color::DEBUG,
        }
    );
}
// from_rotation_y(180.0f32.to_radians())
