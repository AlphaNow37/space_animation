// consts from https://lygia.xyz/v1.2.0/color/space/oklab2rgb
const OKLAB_TO_LMS : mat3x3<f32>  = mat3x3<f32>(
    vec3f(1.0, 1.0, 1.0),
    vec3f(0.3963377774, -0.1055613458, -0.0894841775),
    vec3f(0.2158037573, -0.0638541728, -1.2914855480)
);
const LMS3_TO_SRGB : mat3x3<f32>  = mat3x3<f32>(
    vec3f(4.0767245293, -1.2684380046, -0.0041960863),
    vec3f(-3.3077115913, 2.6097574011, -0.7034186147f),
    vec3f(0.2309699292, -0.3413193965, 1.7076147010)
);

fn oklab_to_srgb(oklab: vec3<f32>) -> vec3<f32> {
    let lms = OKLAB_TO_LMS * oklab;
    return LMS3_TO_SRGB * (lms * lms * lms);
}
fn gamma_to_linear(channel: f32) -> f32 {
    if (channel <= 0.04045) {
        return channel * 0.0773993808; // 1.0 / 12.92;
     } else {
        return pow((channel + 0.055) / 1.055, 2.4);
    }
}
fn srgb_to_rgb(srgb: vec3<f32>) -> vec3<f32> {
    return vec3(
        gamma_to_linear(srgb.r),
        gamma_to_linear(srgb.g),
        gamma_to_linear(srgb.b)
    );
}
fn oklab_to_rgb(oklab: vec3<f32>) -> vec3<f32> {
    return srgb_to_rgb(oklab_to_srgb(oklab));
}
fn frag_out(oklab: vec3<f32>) -> vec4<f32> {
    return vec4(oklab_to_rgb(oklab), 1.);
}
