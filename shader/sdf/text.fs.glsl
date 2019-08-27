layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 1) uniform texture2D sdf_texture;
layout(set = 0, binding = 2) uniform sampler sdf_sampler;
layout(set = 0, binding = 3) uniform DrawUniform {
    vec4 stroke_color;
    vec2 mask_n_gamma;
};

// 反走样
float aastep(float value, float mask)
{
    float afwidth = length(vec2(dFdx(value), dFdy(value))) * 0.70710678118654757  ;
    return smoothstep(mask - afwidth, mask + afwidth, value);
}

float aastep2(float value, float mask)
{
    return smoothstep(mask - 0.055, mask + 0.055, value);
}

vec4 lerp(vec4 a, vec4 b, float w) {
    return a + w * (b - a);
}

void main(void)
{
    float tex_gray = texture(sampler2D(sdf_texture, sdf_sampler), uv).r;
    // // 反转一下数值
    // tex_gray = (1.0 - tex_gray);

    float alpha = aastep2(tex_gray, mask_n_gamma[0]);
    vec4 stroke_color = vec4(stroke_color.rgb, alpha);

    // if (show_outline) {
    //     vec4 outline_color = outline;
    //     outline_color.a = aastep2(tex_gray, outline_mask);
    //     stroke_color = lerp(outline_color, stroke_color, alpha);
    // }
    // if (alpha < 0.75) {
    //     vec4 shadow_color = vec4(0.7, 0.1, 0.1, 1.0);
    //     shadow_color.a = smoothstep(0.753 - 0.45, 0.753, alpha);
    //     stroke_color = shadow_color; //lerp(shadow_color, stroke_color, alpha);
    // }
    frag_color = stroke_color;
    // frag_color = vec4(vec3(tex_gray), 1.0);
}
