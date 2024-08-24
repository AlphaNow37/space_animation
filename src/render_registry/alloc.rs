use crate::world::stores::StoreLabel;
use super::pipelines::PipelineLabel;

#[derive(Default)]
pub struct BufferAllocator {
    instance: [usize; PipelineLabel::COUNT],
    store: [usize; StoreLabel::COUNT],
}
impl BufferAllocator {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_instance_count(&self, pipe: PipelineLabel) -> usize {
        self.instance[pipe as usize]
    }
    pub fn get_store_count(&self, store: StoreLabel) -> usize {
        self.store[store as usize]
    }
    pub fn alloc_instance(&mut self, pipe: PipelineLabel, nb_instance: usize) {
        self.instance[pipe as usize] += nb_instance;
    }
    pub fn alloc_store(&mut self, store: StoreLabel, nb_stored: usize) {
        self.store[store as usize] += nb_stored;
    }
}
