use rand::Rng;

use crate::{
    datastrutures::graph::Graph,
    utils::{Length, VectorSpace},
};

use super::graph::LinkGraph;

/// pairs:
///    C
///    |
///    |
/// A--+--B
///    |
///    |
///    D

pub struct DimensionParam<T> {
    a: T,
    b: T,
    point_amount: usize,
    // Average variation from the center in a [0; +oo] scale. 0=base place, 1=until neighboor
    mean_variation: f32,
}
pub struct SampleLinkPointParam<const NDIM: usize, T> {
    pub dims: [DimensionParam<T>; NDIM],
}

impl<const NDIM: usize, T: VectorSpace + Length> SampleLinkPointParam<NDIM, T> {
    pub fn eval(&self, rng: &mut impl Rng) -> (LinkGraph, Vec<T>) {
        let offsets = [0; NDIM];
        // for i in
        let size: usize = self.dims.iter().map(|d| d.point_amount).product();
        let mut points = vec![T::ZERO; size];
        todo!()
    }
}
