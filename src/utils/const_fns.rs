use std::f32::consts::{TAU, PI};

const fn eval_taylor<const N: usize>(derivatives: [f32; N], x: f32) -> f32 {
    let mut i = 0;
    let mut powx = 1.;  // x^i
    let mut fact = 1;  // i!
    let mut res = 0.;  // sum
    while i < N {
        res += derivatives[i] * powx / fact as f32;
        i += 1;
        fact *= i;
        powx *= x;
    }
    res
}

pub const fn sin(x: f32) -> f32 {
    eval_taylor([0., 1., 0., -1., 0., 1., 0., -1., 0., 1., 0., -1.], x)
}
pub const fn cos(x: f32) -> f32 {
    eval_taylor([1., 0., -1., 0., 1., 0., -1., 0., 1., 0., -1., 0.], x)
}

#[test]
fn test() {
    dbg!(cos(TAU));
    dbg!(cos(TAU / 4.));
    dbg!(sin(TAU));
    dbg!(sin(TAU / 4.));
}

// pub const fn sin(x: f32) -> f32 {
//     const fn sin_0_pi(x: f32) -> f32 {
//         let x2 = x*x;
//         x * (1. + x2 * (-1. + x2 / 20.) / 6.)
//     }
//     const fn sin_positive(x: f32) -> f32 {
//         let x = x % TAU;
//         if x > PI {sin_0_pi(TAU - x)} else {sin_0_pi(x)}
//     }
//     const fn sin_general(x: f32) -> f32 {
//         if x < 0. {
//             -sin_positive(-x)
//         } else {
//             sin_positive(x)
//         }
//     }
//     sin_general(x)
// }
//
// pub const fn cos(x: f32) -> f32 {
//     sin(x + PI/2.)
// }
