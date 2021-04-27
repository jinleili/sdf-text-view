#include "sdf/layout_and_fn.wgsl"

[[stage(compute), workgroup_size(16, 16)]]
fn main([[builtin(global_invocation_id)]] GlobalInvocationID: vec3<u32>) {
  let uv : vec2<i32> = vec2<i32>(GlobalInvocationID.xy);
  if (uv.x >= params.info.x || uv.y >= params.info.y) {
    return;
  }

  let pixel_index : i32 = get_pixel_index2(uv);
  var luma: f32;
  if (params.info[2] == 2) {
    luma = textureLoad(input_pic, uv).r;
    // init front && background distance fields
    let convert_luma: f32 = 1.0 - luma;
    if (convert_luma > 0.949) {
      g_front.data[pixel_index] = INF;
      g_background.data[pixel_index] = 0.0;
    } elseif (convert_luma < 0.1) {
      g_front.data[pixel_index] = 0.0;
      g_background.data[pixel_index] = INF;
    } else {
      g_front.data[pixel_index] = pow(max(0.0, convert_luma - 0.5), 2.0);
      g_background.data[pixel_index] = pow(max(0.0, 0.5 - convert_luma), 2.0);
    }
  } else {
    // output final distans fields
    let dis: f32 = sqrt(g_background.data[pixel_index]) - sqrt(g_front.data[pixel_index]);
    luma = clamp((1.0 - (dis / 22.0 + 0.25)), 0.0, 1.0);
  }
  textureStore(output_pic, uv, vec4<f32>(luma, 0.0, 0.0, 0.0));
}