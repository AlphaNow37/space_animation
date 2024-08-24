use crate::world::world::GlobalStore;
use super::pipelines::PipelineLabel;

#[derive(Default)]
pub struct BufferAllocator {
    instance: [usize; PipelineLabel::COUNT],
    pub store: [usize; GlobalStore::COUNT],
}
impl BufferAllocator {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_count(&self, pipe: PipelineLabel) -> usize {
        self.instance[pipe as usize]
    }
    pub fn alloc_instance(&mut self, pipe: PipelineLabel, nb_instance: usize) {
        self.instance[pipe as usize] += nb_instance;
    }
    pub fn alloc_store(&mut self, store: usize, nb_stored: usize) {
        self.store[store] += nb_stored;
    }
}
