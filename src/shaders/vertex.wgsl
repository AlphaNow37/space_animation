
struct TriVertex {
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) uv: vec3<f32>,
}

struct VertexInput {
    // vertex: TriVertex,
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) uv: vec3<f32>,
    @location(3) col: vec4<f32>,
};

@vertex
fn vs_tri_color1(
    @builtin(vertex_index) in_vertex_index: u32,
    in: VertexInput,
) -> FragmentInput {
    var out: FragmentInput;
    out.clip_position = camera * vec4<f32>(in.pos, 1.0);
    out.col = in.col.xyz;
    out.uv = in.uv;
    out.normal = in.normal.xyz;
    out.delta_pos = in.pos - camera_transform[3].xyz;
    return out;
}
