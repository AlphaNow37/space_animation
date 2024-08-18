
@group(0) @binding(0)
var<uniform> time: f32;

@group(0) @binding(1)
var<uniform> camera: mat4x4<f32>;

@group(0) @binding(2)
var<uniform> eye_dir: vec3<f32>;

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) col: vec4<f32>,
    @location(2) uv: vec3<f32>,
    @location(3) normal: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) col: vec4<f32>,
    @location(1) uv: vec3<f32>,
    @location(2) normal: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera * vec4<f32>(in.pos, 1.0);
    out.col = in.col;
    out.uv = in.uv;
    out.normal = in.normal.xyz;
    return out;
}
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var z = in.clip_position.z;
    var dot = abs(dot(eye_dir, in.normal));
    return in.col * ((1.-z)/(1.-z*0.8)) * (dot * 0.5 + 0.5);
}
