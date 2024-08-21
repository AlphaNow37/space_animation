
//struct TriVertex {
//    @location(0) pos: vec3<f32>,
//    @location(1) normal: vec4<f32>,
//    @location(2) uv: vec3<f32>,
//}

struct TriVertexCol1 {
    // vertex: TriVertex,
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) uv: vec3<f32>,
    @location(3) col: vec4<f32>,
};
struct TriVertexCol2 {
    // vertex: TriVertex,
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) uv: vec3<f32>,
    @location(3) col1: vec4<f32>,
    @location(4) col2: vec4<f32>,
};

@vertex
fn vs_tri_color1(
    in: TriVertexCol1,
) -> FragInputCol1 {
    var out: FragInputCol1;
    out.clip_position = camera * vec4<f32>(in.pos, 1.0);
    out.col = in.col.xyz;
    out.uv = in.uv;
    out.normal = in.normal.xyz;
    out.delta_pos = in.pos - camera_transform[3].xyz;
    return out;
}

@vertex
fn vs_tri_color2(
    in: TriVertexCol2,
) -> FragInputCol2 {
    var out: FragInputCol2;
    out.clip_position = camera * vec4<f32>(in.pos, 1.0);
    out.col1 = in.col1.xyz;
    out.col2 = in.col2.xyz;
    out.uv = in.uv;
    out.normal = in.normal.xyz;
    out.delta_pos = in.pos - camera_transform[3].xyz;
    return out;
}
