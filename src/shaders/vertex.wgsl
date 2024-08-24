
struct LocalGlobalMatrix {
    @location(1) local_global_material: vec3<u32>,
}

struct Pos2Vertex {
    @location(20) pos: vec2<f32>,
}
struct Pos3Vertex {
    @location(20) pos: vec3<f32>,
}

@vertex
fn vs_tri(
    @location(0) pos_global: vec4<u32>,
    @location(3) material: u32,
    @builtin(vertex_index) vertex_index: u32,
) -> FragInput {
    var out: FragInput;

    let global: mat4x4<f32> = matrices[pos_global.w];
    let local_a: vec4<f32> = vec4(vecs3[pos_global.x].xyz, 1.);
    let local_b: vec4<f32> = vec4(vecs3[pos_global.y].xyz, 1.);
    let local_c: vec4<f32> = vec4(vecs3[pos_global.z].xyz, 1.);
    let global_a: vec4<f32> = global * local_a;
    let global_b: vec4<f32> = global * local_b;
    let global_c: vec4<f32> = global * local_c;

    var normal: vec3<f32> = cross(global_a.xyz-global_b.xyz, global_a.xyz-global_c.xyz);
    normal = normal / length(normal);

    if(vertex_index == 0) {
        out.clip_position = camera * global_a;
        out.delta_pos = global_a.xyz - camera_transform[3].xyz;
        out.uv = local_a.xyz;
    } else if(vertex_index == 1) {
        out.clip_position = camera * global_b;
        out.delta_pos = global_b.xyz - camera_transform[3].xyz;
        out.uv = local_b.xyz;
    } else if(vertex_index == 2) {
        out.clip_position = camera * global_c;
        out.delta_pos = global_c.xyz - camera_transform[3].xyz;
        out.uv = local_c.xyz;
    } else {
        out.clip_position = vec4(1., 1., 0., 1.);
    }
    out.normal = normal.xyz;
    out.mat_id = material;
    return out;
}

@vertex
fn vs_sphere(
    in_pos: Pos3Vertex,
    @location(1) local_global_material: vec3<u32>,
) -> FragInput {
    var out: FragInput;

    let local: mat4x4<f32> = matrices[local_global_material.x];
    let global: mat4x4<f32> = matrices[local_global_material.y];

    let normal = global * local * vec4(in_pos.pos.xyz, 0.);
    let local_pos = local * vec4(in_pos.pos.xyz, 1.);
    let global_pos = global * local_pos;
    out.uv = local_pos.xyz;
    out.clip_position = camera * global_pos;
    out.delta_pos = global_pos.xyz - camera_transform[3].xyz;
    out.normal = normal.xyz / length(normal.xyz);
    out.mat_id = local_global_material.z;
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

@vertex
fn vs_polynomial4x4(
    @location(2) global_facts_material: vec3<u32>,
) -> FragInput {
    var out: FragInput;
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

    return out;
}
