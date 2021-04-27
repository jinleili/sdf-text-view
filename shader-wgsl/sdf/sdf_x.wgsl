#include "sdf/layout_and_fn.wgsl"

[[stage(compute), workgroup_size(16, 1)]]
fn main([[builtin(global_invocation_id)]] GlobalInvocationID: vec3<u32>) {
    let uv: vec2<i32> = vec2<i32>(GlobalInvocationID.xy);
    if (uv.x >= params.info.x) {
        return;
    }

    // transform along columns
    sdf1d(uv.x, params.info.x, params.info.y, uv.x, params.info.x + 1);
}