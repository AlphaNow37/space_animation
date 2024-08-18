use glam::{Affine3A, Mat3A, Vec3A};

use super::variator::{new_typed_variator, Variator};

new_typed_variator!(
    WithRotation(P: Vec3A, R: Mat3A) => Affine3A {Affine3A {translation: P, matrix3: R}}
);

new_typed_variator!(
    ProjectPoint(A: Affine3A, P: Vec3A) => Vec3A {
        A.transform_point3a(P)
    }
);

new_typed_variator!(Translation(A: Affine3A) => Vec3A {A.translation});
