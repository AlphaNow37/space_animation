use crate::math::{Angle, Dir, Polynomial, Transform, Vec2, Vec3, Vec4};
use crate::utils::array_key;
use crate::world::primitives::camera::Camera;
use crate::world::primitives::color::Color;

macro_rules! make_system {
    (
        $(
            $attr: ident : $prim_ty: ident $(($store_method: ident, $len_method: ident))?
        );*
        $(;)?
    ) => {
        $(
            impl Variator for $prim_ty {
                type Item=$prim_ty;
                fn hash_var(&self) -> u32 {
                    self.gen_hash()
                }
                fn eq_var(&self, other: &Self) -> bool {
                    self == other
                }
                fn update(&self, _world: &World) -> $prim_ty {
                    *self
                }
            }
            impl WorldPrimitive for $prim_ty {
                fn alloc(world: &mut World, size: usize) -> usize {
                    let idx = world.$attr.len();
                    world.$attr.reserve(size);
                    for _ in 0..size {
                        world.$attr.push(Cell::new(Self::default()));
                    }
                    idx
                }
                fn get(world: &World, index: usize) -> Self {
                    world.$attr[index].get()
                }
                fn set(world: &World, index: usize, value: Self) {
                    world.$attr[index].set(value);
                }
                fn sets<const N: usize>(world: &World, index: usize, values: [Self; N]) {
                    for i in 0..values.len() {
                        Self::set(world, index+i, values[i]) // i hope this gets optimised away
                    }
                }
            }
        )*

        array_key!(
            pub enum Primitives {
                $(
                    $prim_ty,
                )*
            }
        )

        #[derive(Clone, Debug, Default)]
        pub struct PrimitiveMap<T> {
            $(
                $attr: T,
            )*
        }
        impl<T> PrimitiveMap<T> {
            pub fn get(&self, key: Primitives) -> &T {
                match key {
                    $(
                        Primitives::$prim_ty => self.$attr,
                    )*
                }
            }
            pub fn set(&mut self, key: Primitives, value: T) {
                match key {
                    $(
                        Primitives::$prim_ty => {self.$attr = value},
                    )*
                }
            }
        }

        #[derive(Default, Debug, Clone)]
        pub struct PrimitiveRegisters {
            $(
                $attr: Vec<Cell<$prim_ty>>,
            )*
        }

        impl World {
            pub fn new() -> Self {
                Self {
                    $(
                        $attr: Vec::new(),
                    )*
                    directives: Vec::new(),
                    settings: WorldSettings::default(),
                    variators: Vec::new(),
                    variators_cache: HashMap::new(),
                    sub_worlds: Vec::new(),
                }
            }
        }
    };
}

type F32 = f32;
type Color2 = (Color, Color);
type Polynomial4x4 = Polynomial<Vec3, 4, 4>;

make_system!(
    vec3: Vec3;
    transform: Transform;
    color: Color;
    f32: F32;
    vec2: Vec2;
    camera: Camera;
    angle: Angle;
    dir: Dir;
    vec4: Vec4;
    color2: Color2;
    polynomial4x4: Polynomial4x4;
);
