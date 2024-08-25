use bytemuck::Pod;
use crate::math::{Dir, Polynomial, Transform, Vec2, Vec3, Vec4};
use crate::utils::Zero;
use crate::world::primitives::color::Color;

pub trait AsStrorageStruct {
    type S: Pod;
    fn as_strorage_struct(&self) -> Self::S;
}

impl AsStrorageStruct for f32 {
    type S = [f32; 4];
    fn as_strorage_struct(&self) -> Self::S {
        [*self, 0., 0., 0.]
    }
}
impl AsStrorageStruct for Vec2 {
    type S = [f32; 4];
    fn as_strorage_struct(&self) -> Self::S {
        [self.x(), self.y(), 0., 0.]
    }
}
impl AsStrorageStruct for Vec3 {
    type S = [f32; 4];
    fn as_strorage_struct(&self) -> Self::S {
        self.0.to_array()
    }
}
impl AsStrorageStruct for Vec4 {
    type S = [f32; 4];
    fn as_strorage_struct(&self) -> Self::S {
        self.to_array()
    }
}
impl AsStrorageStruct for Color {
    type S = [f32; 4];
    fn as_strorage_struct(&self) -> Self::S {
        let arr = self.to_array();
        [arr[0], arr[1], arr[2], 0.]
    }
}
impl AsStrorageStruct for Dir {
    type S = [f32; 4];
    fn as_strorage_struct(&self) -> Self::S {
        self.0.to_array()
    }
}
impl AsStrorageStruct for Transform {
    type S = [f32; 16];
    fn as_strorage_struct(&self) -> Self::S {
        self.to_mat4().to_array()
    }
}
impl<T: AsStrorageStruct+Copy, const N: usize, const M: usize> AsStrorageStruct for Polynomial<T, N, M> where T::S: Zero {
    type S = [[T::S; N]; M];
    fn as_strorage_struct(&self) -> Self::S {
        self.map_comp(|t| t.as_strorage_struct()).0
    }
}

impl<A: AsStrorageStruct, B: AsStrorageStruct<S=A::S>> AsStrorageStruct for (A, B) {
    type S = [A::S; 2];
    fn as_strorage_struct(&self) -> Self::S {
        [self.0.as_strorage_struct(), self.1.as_strorage_struct()]
    }
}
