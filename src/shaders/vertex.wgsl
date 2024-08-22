
struct TriVertex {
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) uv: vec3<f32>,
}

struct LocalMatrix {
    @location(0) local1: vec4<f32>,
    @location(1) local2: vec4<f32>,
    @location(2) local3: vec4<f32>,
    @location(3) local4: vec4<f32>,
}

struct GlobalMatrix {
    @location(4) global1: vec4<f32>,
    @location(5) global2: vec4<f32>,
    @location(6) global3: vec4<f32>,
    @location(7) global4: vec4<f32>,
}

//struct Polynomial4x4 {
//    @location(30) p00: vec3<f32>,
//    @location(31) p01: vec3<f32>,
//    @location(32) p02: vec3<f32>,
//    @location(33) p03: vec3<f32>,
//    @location(34) p10: vec3<f32>,
//    @location(35) p11: vec3<f32>,
//    @location(36) p12: vec3<f32>,
//    @location(37) p13: vec3<f32>,
//    @location(38) p20: vec3<f32>,
//    @location(39) p21: vec3<f32>,
//    @location(40) p22: vec3<f32>,
//    @location(41) p23: vec3<f32>,
//    @location(42) p30: vec3<f32>,
//    @location(43) p31: vec3<f32>,
//    @location(44) p32: vec3<f32>,
//    @location(45) p33: vec3<f32>,
//}

struct Col1 {
    @location(10) col: vec4<f32>,
};

struct Col2 {
    @location(10) col1: vec4<f32>,
    @location(11) col2: vec4<f32>,
};

struct Pos2Vertex {
    @location(20) pos: vec2<f32>,
}
struct Pos3Vertex {
    @location(20) pos: vec3<f32>,
}

fn local_matrix(in: LocalMatrix) -> mat4x4<f32> {
    return mat4x4(in.local1, in.local2, in.local3, in.local4);
}
fn global_matrix(in: GlobalMatrix) -> mat4x4<f32> {
    return  mat4x4(in.global1, in.global2, in.global3, in.global4);
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
    in_local: LocalMatrix,
    in_global: GlobalMatrix,
    in_col: Col1,
    in_pos: Pos3Vertex,
) -> FragInputCol1 {
    var out: FragInputCol1;

    let local: mat4x4<f32> = local_matrix(in_local);
    let global: mat4x4<f32> = global_matrix(in_global);

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

//fn project_poly(poly: Polynomial4x4, pt: Pos2Vertex) -> vec3<f32> {
//    let u: f32 = pt.pos.x;
//    let v: f32 = pt.pos.x;
//
//    return (
//        (((((poly.p33 * u + poly.p32) * u + poly.p31) * u + poly.p30) * v
//         +(((poly.p23 * u + poly.p22) * u + poly.p21) * u + poly.p20)) * v
//         +(((poly.p13 * u + poly.p12) * u + poly.p11) * u + poly.p10)) * v
//         +(((poly.p03 * u + poly.p02) * u + poly.p01) * u + poly.p00)
//    );
//}
//
//@vertex
//fn vs_polynomial4x4_color1(
//    in_global: GlobalMatrix,
//    in_poly: Polynomial4x4,
//    in_pos: Pos2Vertex,
//    in_col: Col1,
//) -> FragInputCol1 {
//    var out: FragInputCol1;
//
//    let global: mat4x4<f32> = global_matrix(in_global);
//
//    let pos = project_poly(in_poly, in_pos);
//    let global_pos = global * vec4(pos, 1.);
//
//    out.clip_position = camera * global_pos;
//    out.col = in_col.col.xyz;
//    out.uv = pos;
//    out.delta_pos = global_pos.xyz - camera_transform[3].xyz;
//    out.normal = vec3(0., 0., 1.);
//
//    return out;
//}
@vertex
fn vs_polynomial4x4_color1() -> FragInputCol1 {
    var out: FragInputCol1;
    return out;
}
