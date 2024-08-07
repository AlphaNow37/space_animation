
@group(0) @binding(0)
var<uniform> time: vec2<f32>;

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) col: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) col: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.pos.xy, 0.0, 1.0);
    out.col = in.col;
    return out;
}
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.col.xyz, 1.);
}
