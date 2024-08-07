use std::ops::Range;

use super::pipelines::PipelineLabel;

#[derive(Default)]
pub struct BuffersAllocPosition {
    vertex_index: [(usize, usize); PipelineLabel::COUNT],
}
impl BuffersAllocPosition {
    pub fn new() -> Self {Self::default()}
    pub fn get(&self, pipe: PipelineLabel) -> (usize, usize) {self.vertex_index[pipe as usize]}
    pub fn alloc(&mut self, pipe: PipelineLabel, vertex_size: usize, index_size: usize) -> Position {
        let (start_vertex, start_index) = &mut self.vertex_index[pipe as usize];
        let pos = Position {
            pipe_label: pipe,
            index_bound: *start_index..*start_index+index_size,
            vertex_bound: *start_vertex..*start_vertex+vertex_size,
        };
        *start_vertex = pos.vertex_bound.end;
        *start_index = pos.index_bound.end;
        pos
    }
}

pub struct Position {
    pipe_label: PipelineLabel,
    vertex_bound: Range<usize>,
    index_bound: Range<usize>,
}
