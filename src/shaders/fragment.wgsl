
struct FragInput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) delta_pos: vec3<f32>,
    @location(3) mat_id: u32,
};

@fragment
fn fs_none(in: FragInput) -> @location(0) vec4<f32> {
    var col: vec3<f32> = vec3(0.35, 0.12, -0.12); // Purple
    col.x *= pass_all(in.normal, in.clip_position.z, in.delta_pos);
    return frag_out(col);
}

@fragment
fn fs_uniform(in: FragInput) -> @location(0) vec4<f32> {
    var col: vec3<f32>= colors[in.mat_id];
    col.x *= pass_all(in.normal, in.clip_position.z, in.delta_pos);
    return frag_out(col);
}

@fragment
fn fs_sponge(in: FragInput) -> @location(0) vec4<f32> {
    var col: vec3<f32> = colors2[in.mat_id*2];
    if(is_on_sponge(in.uv)) {
        col = colors2[in.mat_id*2+1];
    }
    col.x *= pass_all(in.normal, in.clip_position.z, in.delta_pos);
    return frag_out(col);
}
