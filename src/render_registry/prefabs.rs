use std::sync::LazyLock;
use tracing::info;
use crate::math::Vec3;
use crate::render_registry::vertex::{Pos2Vertex, Pos3Vertex, VertexBufferLabel};
use crate::utils::VectorSpace;

#[derive(Clone, Copy, Debug)]
pub struct VertexPoss {
    pub label: VertexBufferLabel,
    pub content: &'static [u32],
    pub len: u32,
}

// Looks good, like a rose
// pub static CIRCLE_POS: LazyLock<(u32, &'static [u32])> = LazyLock::new(|| {
//     let mut vertexes = vec![
//         Vec3::X, -Vec3::X,
//         Vec3::Y, -Vec3::Y,
//         Vec3::Z, -Vec3::Z,
//     ];
//     let mut idxs = vec![
//         [0, 2, 4],
//         [0, 3, 4],
//         [1, 2, 4],
//         [1, 3, 4],
//         [0, 2, 5],
//         [0, 3, 5],
//         [1, 2, 5],
//         [1, 3, 5],
//     ];
//     let mut new_idxs = vec![];
//     for _ in 0..CIRCLE_ITERATIONS {
//         for &[a, b, c] in &idxs {
//             let (va, vb, vc) = (vertexes[a], vertexes[b], vertexes[c]);
//             let (vmida, vmidb, vmidc) = (vb.mid(vc), va.mid(vc), va.mid(vb));
//             let (mida, midb, midc) = (vertexes.len(), vertexes.len()+1, vertexes.len()+2);
//             vertexes.extend([vmida.with_len(1.), vmidb.with_len(1.), vmidc.with_len(1.)]);
//             new_idxs.extend([
//                 [a, midc, b],
//                 [a, midb, c],
//                 [b, mida, c],
//                 [mida, midb, midc],
//             ])
//         }
//         (new_idxs, idxs) = (idxs, new_idxs);
//         new_idxs.clear();
//     }
//
//     let vs = idxs.into_iter()
//         .flat_map(|is| is.map(|i| vertexes[i]))
//         .map(|v| PosVertex {pos: v.to_array()})
//         .collect::<Vec<_>>();
//     (vs.len() as u32, bytemuck::cast_slice(vs.leak()))
// });


const CIRCLE_ITERATIONS: usize = 3;
pub static CIRCLE_POS: LazyLock<VertexPoss> = LazyLock::new(|| {
    let mut vertexes = vec![
        Vec3::X, -Vec3::X,
        Vec3::Y, -Vec3::Y,
        Vec3::Z, -Vec3::Z,
    ];
    let mut idxs = vec![
        [0, 2, 4],
        [0, 3, 4],
        [1, 2, 4],
        [1, 3, 4],
        [0, 2, 5],
        [0, 3, 5],
        [1, 2, 5],
        [1, 3, 5],
    ];
    let mut new_idxs = vec![];
    for _ in 0..CIRCLE_ITERATIONS {
        for &[a, b, c] in &idxs {
            let (va, vb, vc) = (vertexes[a], vertexes[b], vertexes[c]);
            let (vmida, vmidb, vmidc) = (vb.mid(vc), va.mid(vc), va.mid(vb));
            let (mida, midb, midc) = (vertexes.len(), vertexes.len()+1, vertexes.len()+2);
            vertexes.extend([vmida.with_len(1.), vmidb.with_len(1.), vmidc.with_len(1.)]);
            new_idxs.extend([
                [mida, c, midb],
                [mida, b, midc],
                [midb, a, midc],
                [mida, midb, midc],
            ])
        }
        (new_idxs, idxs) = (idxs, new_idxs);
        new_idxs.clear();
    }

    let vs = idxs.into_iter()
        .flat_map(|is| is.map(|i| vertexes[i]))
        .map(|v| Pos3Vertex {pos: v.to_array()})
        .collect::<Vec<_>>();
    info!("Created circle, size={}", vs.len());
    VertexPoss {
        len: vs.len() as u32,
        label: VertexBufferLabel::Pos3,
        content: bytemuck::cast_slice(vs.leak()),
    }
});

const FLAT_SUBDIVISIONS: usize = 10;
pub static FLAT_POS: LazyLock<VertexPoss> = LazyLock::new(|| {
    let vs = (0..FLAT_SUBDIVISIONS)
        .flat_map(|x| (0..FLAT_SUBDIVISIONS).map(move |y| (x, y)))
        .flat_map(|(x, y)| [(x, y), (x+1, y), (x, y+1), (x+1, y), (x, y+1), (x+1, y+1)])
        .map(|(x, y)| Pos2Vertex {pos: [x as f32 / FLAT_SUBDIVISIONS as f32, y as f32 / FLAT_SUBDIVISIONS as f32]})
        .collect::<Vec<_>>();

    VertexPoss {
        len: vs.len() as u32,
        label: VertexBufferLabel::Pos2,
        content: bytemuck::cast_slice(vs.leak()),
    }
});
