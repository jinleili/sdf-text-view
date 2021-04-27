#include "sdf/layout_and_fn.wgsl"

[[stage(compute), workgroup_size(1, 16)]]
fn main([[builtin(global_invocation_id)]] GlobalInvocationID: vec3<u32>) {
    let uv: vec2<i32> = vec2<i32>(GlobalInvocationID.xy);
    if (uv.y >= params.info.y ) {
        return;
    }
    
    // transform along rows
    sdf1d(uv.y * params.info.x, 1, params.info.x, uv.y * (params.info.x + 1), 1);
}