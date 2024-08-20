use crate::math::{Transform, Vec3};

use super::variators::variator::{new_typed_variator, Variator};

new_typed_variator!(
    ProjectPoint(A: Transform, P: Vec3) => Vec3 {
        A.tr_point(P)
    }
);

new_typed_variator!(Translation(A: Transform) => Vec3 {A.trans()});
