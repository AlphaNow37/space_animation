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

// pub const fn factorial(n: usize) -> usize {
//     let mut k = 0;
//     let mut fact = 1;
//     while k < n {
//         k += 1;
//         fact *= k;
//     }
//     fact
// }

#[test]
fn test() {
    dbg!(cos(TAU));
    dbg!(cos(TAU / 4.));
    dbg!(sin(TAU));
    dbg!(sin(TAU / 4.));
}
