
@group(0) @binding(0)
var<uniform> time: vec2<f32>;

struct VertexInput {
    @location(0) col: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) col: f32,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.col = in.col;
    return out;
}
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(time.x / time.y, in.col, 0., 1.);
}
