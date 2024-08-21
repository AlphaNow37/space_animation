
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
        // let pos = Position {
        //     pipe_label: pipe,
        //     index_bound: *start_index..*start_index + nb_index,
        //     vertex_bound: *start_vertex..*start_vertex + nb_vertex,
        // };
        // *start_vertex = pos.vertex_bound.end;
        // *start_index = pos.index_bound.end;
        // pos
    }
}

// #[derive(Debug, Clone)]
// pub struct Position {
//     pub pipe_label: PipelineLabel,
//     pub vertex_bound: Range<usize>,
//     pub index_bound: Range<usize>,
// }
