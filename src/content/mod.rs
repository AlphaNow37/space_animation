use glam::{Affine3A, Mat3, Mat3A, Vec3, Vec3A};
use crate::{models::put_axis, world::{camera::TrackCamera, color::Color, combinators::Interpolate, material::UniformTri, point::WithRotation, rotation::Angle, shape::{Cube, CubeSphere, Triangle}, variator::Variator, world::World}};

pub fn build(world: &mut World) {
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
    })
}
// from_rotation_y(180.0f32.to_radians())
