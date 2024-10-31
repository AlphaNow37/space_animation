use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::world::primitives::primitives::{PrimitiveMap, PrimitiveRegisters};
use crate::world::variators::saved_variator::SavedVariator;
use crate::world::visuals::VisualDirective;
use crate::world::world::{World, WorldSettings};

pub struct WorldBuilder {
    register_sizes: PrimitiveMap<usize>,
    variators: Vec<Box<dyn SavedVariator>>,
    variators_cache: HashMap<u32, usize>,
    directives: Vec<Box<dyn VisualDirective>>,
}
