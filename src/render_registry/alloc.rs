use crate::render_registry::materials::MaterialType;
use crate::render_registry::vertex::VertexType;
use crate::world::stores::StoreLabel;

#[derive(Default)]
pub struct BufferAllocator {
    instance: [[usize; MaterialType::COUNT]; VertexType::COUNT],
    store: [usize; StoreLabel::COUNT],
}
impl BufferAllocator {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_instance_count(&self, vertex: VertexType, material: MaterialType) -> usize {
        self.instance[vertex as usize][material as usize]
    }
    pub fn get_store_count(&self, store: StoreLabel) -> usize {
        self.store[store as usize]
    }
    pub fn alloc_instance(&mut self, vertex: VertexType, material: MaterialType, nb_instance: usize) {
        self.instance[vertex as usize][material as usize] += nb_instance;
    }
    pub fn alloc_store(&mut self, store: StoreLabel, nb_stored: usize) {
        self.store[store as usize] += nb_stored;
    }
}
