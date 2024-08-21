
use super::pipelines::PipelineLabel;

#[derive(Default)]
pub struct BuffersAllocPosition {
    vertex_index: [(usize, usize); PipelineLabel::COUNT],
}
impl BuffersAllocPosition {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_count(&self, pipe: PipelineLabel) -> (usize, usize) {
        self.vertex_index[pipe as usize]
    }
    pub fn alloc(&mut self, pipe: PipelineLabel, nb_vertex: usize, nb_index: usize) {
        let (start_vertex, start_index) = &mut self.vertex_index[pipe as usize];
        *start_vertex += nb_vertex;
        *start_index += nb_index;
    }
}
