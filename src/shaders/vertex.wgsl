
struct TriVertex {
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) uv: vec3<f32>,
}
struct SphereVertex {
    @location(0) local1: vec4<f32>,
    @location(1) local2: vec4<f32>,
    @location(2) local3: vec4<f32>,
    @location(3) local4: vec4<f32>,
    @location(4) global1: vec4<f32>,
    @location(5) global2: vec4<f32>,
    @location(6) global3: vec4<f32>,
    @location(7) global4: vec4<f32>,
}

struct Col1 {
    @location(10) col: vec4<f32>,
};
struct Col2 {
    @location(10) col1: vec4<f32>,
    @location(11) col2: vec4<f32>,
};

struct PosVertex {
    @location(20) pos: vec3<f32>,
}

@vertex
fn vs_tri_color1(
    in_vertex: TriVertex,
    in_col: Col1,
) -> FragInputCol1 {
    var out: FragInputCol1;
    out.clip_position = camera * vec4<f32>(in_vertex.pos, 1.0);
    out.col = in_col.col.xyz;
    out.uv = in_vertex.uv;
    out.normal = in_vertex.normal.xyz;
    out.delta_pos = in_vertex.pos - camera_transform[3].xyz;
    return out;
}

@vertex
fn vs_tri_color2(
    in_vertex: TriVertex,
    in_col: Col2,
) -> FragInputCol2 {
    var out: FragInputCol2;
    out.clip_position = camera * vec4<f32>(in_vertex.pos, 1.0);
    out.col1 = in_col.col1.xyz;
    out.col2 = in_col.col2.xyz;
    out.uv = in_vertex.uv;
    out.normal = in_vertex.normal.xyz;
    out.delta_pos = in_vertex.pos - camera_transform[3].xyz;
    return out;
}

@vertex
fn vs_sphere_color1(
    in_vertex: SphereVertex,
    in_col: Col1,
    in_pos: PosVertex,
) -> FragInputCol1 {
    var out: FragInputCol1;

    let local: mat4x4<f32> = mat4x4(in_vertex.local1, in_vertex.local2, in_vertex.local3, in_vertex.local4);
    let global: mat4x4<f32> = mat4x4(in_vertex.global1, in_vertex.global2, in_vertex.global3, in_vertex.global4);

    let normal = global * local * vec4(in_pos.pos.xyz, 0.);
    let local_pos = local * vec4(in_pos.pos.xyz, 1.);
    let global_pos = global * local_pos;
    out.uv = local_pos.xyz;
    out.clip_position = camera * global_pos;
    out.delta_pos = global_pos.xyz - camera_transform[3].xyz;
    out.col = in_col.col.xyz;
    out.normal = normal.xyz / length(normal.xyz);
    return out;
}
