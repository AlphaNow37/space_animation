
struct FragmentInput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) col: vec3<f32>,
    @location(1) uv: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) delta_pos: vec3<f32>,
};

@fragment
fn fs_uniform(in: FragmentInput) -> @location(0) vec4<f32> {
    var col = in.col;
    col.x *= pass_all(in.normal, in.clip_position.z, in.delta_pos);
    return frag_out(col);
}
