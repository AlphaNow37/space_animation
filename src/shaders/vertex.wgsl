
struct LocalGlobalMatrix {
    @location(1) local_global_material: vec3<u32>,
}

struct Pos2Vertex {
    @location(20) pos: vec2<f32>,
}
struct Pos3Vertex {
    @location(20) pos: vec3<f32>,
}
struct TilePosVertex {
    @location(21) pos: vec2<f32>,
}

fn fvs_tri(
    pos_global: vec4<u32>,
    material: u32,
    vertex_index: u32,
    offset: vec3<f32>,
) -> FragInput {
    var out: FragInput;

    let global: mat4x4<f32> = matrices[pos_global.w];
    let local_a: vec4<f32> = vec4(vecs3[pos_global.x] + offset, 1.);
    let local_b: vec4<f32> = vec4(vecs3[pos_global.y] + offset, 1.);
    let local_c: vec4<f32> = vec4(vecs3[pos_global.z] + offset, 1.);
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
fn vs_tri(
    @location(0) pos_global: vec4<u32>,
    @location(3) material: u32,
    @builtin(vertex_index) vertex_index: u32,
) -> FragInput {
    return fvs_tri(pos_global, material, vertex_index, vec3(0., 0., 0.));
}

@vertex
fn vs_tiled_tri(
    @location(0) pos_global: vec4<u32>,
    @location(3) material: u32,
    @location(4) tiled_matrix: u32,
    @builtin(vertex_index) vertex_index: u32,
    tile_pos: TilePosVertex,
) -> FragInput {
    let matrix: mat4x4<f32> = matrices[tiled_matrix];
    let global: mat4x4<f32> = matrices[pos_global.w];
    let plane: mat4x4<f32> = global * matrix;

    let x_axis = plane[0].xyz; let y_axis = plane[1].xyz; let trans = plane[3].xyz;

    var delta: vec3<f32> = camera_transform[3].xyz-trans;

    let normal = normalize(cross(x_axis, y_axis));
    delta -= dot(normal, delta) * normal;
    
    let xnormal = normalize(cross(x_axis, normal));
    let y = dot(delta, xnormal) / dot(y_axis, xnormal);
    delta -= y*y_axis;
    let x = dot(delta, x_axis) / dot(x_axis, x_axis);

    let offset = matrix * vec4(vec2(round(x), round(y)) + tile_pos.pos, 0., 1.);
    return fvs_tri(pos_global, material, vertex_index % 3, offset.xyz);
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


@vertex
fn vs_cube(
    @location(1) local_global_material: vec3<u32>,
    @builtin(vertex_index) vertex_index: u32,
) -> FragInput {
    var out: FragInput;

    var DIR_A = array<f32, 6>(-1., 1., -1., 1., -1., 1.);
    var DIR_B = array<f32, 6>(-1., -1., 1., -1., 1., 1.);
    var DIR_C = array<f32, 2>(-1., 1.);

    let local: mat4x4<f32> = matrices[local_global_material.x];
    var global: mat4x4<f32> = matrices[local_global_material.y];

    let face = vertex_index / 6;
    let face_dir = face % 3;
    let in_face_id = vertex_index % 6;
    let dir_a = DIR_A[in_face_id];
    let dir_b = DIR_B[in_face_id];
    let dir_c = DIR_C[u32(vertex_index >= 18)];

    var pos: vec3<f32>;
    if(face_dir == 0) {
        pos = vec3(dir_c, dir_a, dir_b);
    } else if(face_dir == 1) {
        pos = vec3(dir_a, dir_c, dir_b);
    } else if(face_dir == 2) {
        pos = vec3(dir_a, dir_b, dir_c);
    }

    let local_pos = local * vec4(pos, 1.);
    let global_pos = global * local_pos;
    out.uv = local_pos.xyz;
    out.clip_position = camera * global_pos;
    out.delta_pos = global_pos.xyz - camera_transform[3].xyz;
    let normal = global[face_dir];
    out.normal = normal.xyz / length(normal.xyz);
    out.mat_id = local_global_material.z;

    return out;
}

fn poly4x4_project_poly(poly: array<vec3<f32>, 16>, u: f32, v: f32) -> vec3<f32> {
    return (
        (((((poly[15] * u + poly[14]) * u + poly[13]) * u + poly[12]) * v
         +(((poly[11] * u + poly[10]) * u + poly[9]) * u + poly[8])) * v
         +(((poly[7] * u + poly[6]) * u + poly[5]) * u + poly[4])) * v
         +(((poly[3] * u + poly[2]) * u + poly[1]) * u + poly[0])
    );
}
fn poly4x4_dir_u(poly: array<vec3<f32>, 16>, u: f32, v: f32) -> vec3<f32> {
    return (
        ((((poly[15] * u * 3. + poly[14]) * u * 2. + poly[13]) * v
         +((poly[11] * u * 3. + poly[10]) * u * 2. + poly[9])) * v
         +((poly[7] * u * 3. + poly[6]) * u * 2. + poly[5])) * v
         +((poly[3] * u * 3. + poly[2]) * u * 2. + poly[1])
    );
}
fn poly4x4_dir_v(poly: array<vec3<f32>, 16>, u: f32, v: f32) -> vec3<f32> {
    return (
        ((((poly[15] * u + poly[14]) * u + poly[13]) * u + poly[12]) * v * 3.
         +(((poly[11] * u + poly[10]) * u + poly[9]) * u + poly[8])) * v * 2.
         +(((poly[7] * u + poly[6]) * u + poly[5]) * u + poly[4])
    );
}

@vertex
fn vs_poly4x4(
    in_pos: Pos2Vertex,
    @location(2) global_facts_material: vec3<u32>,
) -> FragInput {
    var out: FragInput;

    let u: f32 = in_pos.pos.x;
    let v: f32 = in_pos.pos.y;
    let global: mat4x4<f32> = matrices[global_facts_material.x];
    let facts = polys4x4[global_facts_material.y];
    let local_pos = poly4x4_project_poly(facts, u, v);
    let dir_u = poly4x4_dir_u(facts, u, v);
    let dir_v = poly4x4_dir_v(facts, u, v);
    let normal = (global * vec4(cross(dir_u, dir_v), 0.)).xyz;

    let global_pos = global * vec4(local_pos, 1.);

    out.clip_position = camera * global_pos;
    out.uv = local_pos;
    out.delta_pos = global_pos.xyz - camera_transform[3].xyz;
    out.normal = normal / length(normal);
    out.mat_id = global_facts_material.z;

    return out;
}
