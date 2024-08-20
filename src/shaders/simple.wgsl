
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

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) col: vec3<f32>,
    @location(1) uv: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) delta_pos: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera * vec4<f32>(in.pos, 1.0);
    out.col = in.col.xyz;
    out.uv = in.uv;
    out.normal = in.normal.xyz;
    out.delta_pos = in.pos - camera_transform[3].xyz;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var col = in.col;
    col.x *= pass_all(in.normal, in.clip_position.z, in.delta_pos);
    return frag_out(col);
}
