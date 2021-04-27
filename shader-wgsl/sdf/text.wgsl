struct VertexOutput {
  [[location(0)]] uv : vec2<f32>;
  [[builtin(position)]] position : vec4<f32>;
};

[[block]] struct MVPUniform { mvpMatrix : mat4x4<f32>; };
[[group(0), binding(0)]] var<uniform> mvp : MVPUniform;

[[stage(vertex)]] 
fn main(
    [[location(0)]] position: vec3<f32>, 
    [[location(1)]] texCoord: vec2<f32>,
) -> VertexOutput {
  var out : VertexOutput;
  out.position = mvp.mvpMatrix * vec4<f32>(position, 1.0);
  out.uv = texCoord;
  return out;
}

[[block]]
struct DrawUniform {
    stroke_color: vec4<f32>;
    mask_n_gamma: vec2<f32>;
};

[[group(0), binding(1)]] var sdf_texture : texture_2d<f32>;
[[group(0), binding(2)]] var sdf_sampler : sampler;
[[group(0), binding(3)]] var<uniform> draw_uniform : DrawUniform;

// 反走样
fn aastep(value: f32, mask: f32) -> f32 {
    let afwidth: f32 = length(vec2<f32>(dpdx(value), dpdy(value))) * 0.70710678118654757  ;
    return smoothStep(mask - afwidth, mask + afwidth, value);
}

fn aastep2(value: f32, mask: f32) -> f32 {
    return smoothStep(mask - 0.055, mask + 0.055, value);
}

fn lerp(a: vec4<f32>, b: vec4<f32>, w: f32) -> vec4<f32>{
    return a + w * (b - a);
}

[[stage(fragment)]] 
fn main(in : VertexOutput) -> [[location(0)]] vec4<f32> {
    var tex_gray: f32 = textureSample(sdf_texture, sdf_sampler, in.uv).r;
    // 反转一下数值
    // tex_gray = (1.0 - tex_gray);

    let alpha: f32 = aastep(tex_gray, draw_uniform.mask_n_gamma[0]);
    let stroke_color: vec4<f32> = vec4<f32>(draw_uniform.stroke_color.rgb, alpha);
    // let stroke_color: vec4<f32> = vec4<f32>(0.0, 0.1, 0.2, 1.0);
  
    return stroke_color;
}
