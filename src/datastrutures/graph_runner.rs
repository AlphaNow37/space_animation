use super::graph::{Graph, GraphNodeIterable, LinkGraph};

pub struct RunGraph {
    nexts: Vec<Vec<usize>>,
    ancestor_counts: Vec<usize>,
    ancestor_run_buffer: Option<Vec<usize>>,
}
impl RunGraph {
    pub fn from_graph(g: LinkGraph) -> Self {
        let mut ancestors = vec![0; g.size_exact()];
        for n in g.iter_nodes() {
            for end in g.iter_neighboors(n) {
                ancestors[end] += 1;
            }
        }
        Self {
            nexts: g.targets,
            ancestor_counts: ancestors,
            ancestor_run_buffer: Some(Vec::new())
        }
    }
}

pub trait GraphRunContext {
    fn run(&self, id: usize);
}

pub struct GraphRunner<Ctx: GraphRunContext + Sync> {
    graph: RunGraph,
    ctx: Ctx,
}

impl<Ctx: GraphRunContext + Sync> GraphRunner<Ctx> {
    pub fn run(&self, ctx: &Ctx) {
        
    }
}
