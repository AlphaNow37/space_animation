
const SPONGE_ITERATIONS: u32 = 5;
fn is_on_sponge(puv: vec3<f32>) -> bool {
    var uv = abs(puv);
    uv = uv % 1.;
    uv = min(uv, vec3(1., 1., 1.)-uv);
    for(var i: u32=0; i<SPONGE_ITERATIONS; i++) {
        if(u32(uv.x < 1./6.) + u32(uv.y < 1./6.) + u32(uv.z < 1./6.) > 1) {
            return true;
        }
        uv = min(uv, vec3(1./3., 1./3., 1./3.)-uv);
        uv = abs(uv) * 3.;
    }
    return false;
}
