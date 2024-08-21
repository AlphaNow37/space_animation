
struct FragInputCol1 {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) delta_pos: vec3<f32>,
    @location(3) col: vec3<f32>,
};

struct FragInputCol2 {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) delta_pos: vec3<f32>,
    @location(3) col1: vec3<f32>,
    @location(4) col2: vec3<f32>,
};

@fragment
fn fs_uniform(in: FragInputCol1) -> @location(0) vec4<f32> {
    var col = in.col;
    col.x *= pass_all(in.normal, in.clip_position.z, in.delta_pos);
    return frag_out(col);
}

@fragment
fn fs_sponge(in: FragInputCol2) -> @location(0) vec4<f32> {
    var col = in.col1;
    if(is_on_sponge(in.uv)) {
        col = in.col2;
    }
    col.x *= pass_all(in.normal, in.clip_position.z, in.delta_pos);
    return frag_out(col);
}
