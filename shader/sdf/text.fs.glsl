layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 1) uniform texture2D skin_texture;
layout(set = 0, binding = 2) uniform sampler tex_sampler;

const vec4 outline = vec4(1.0, 0.0, 0.0, 1.0);
// Between 0 and 0.5, 0 = thick outline, 0.5 = no outline
const float outline_mask = 0.495;
const float stroke_mask = 0.5;

const bool show_outline = false;
const bool show_shadow = false;

// 反走样
float aastep(float value, float mask)
{
    float afwidth = length(vec2(dFdx(value), dFdy(value))) * 0.70710678118654757 ;
    return smoothstep(mask - afwidth, mask + afwidth, value);
}

vec4 lerp(vec4 a, vec4 b, float w) {
    return a + w * (b - a);
}

void main(void)
{
    float tex_gray = texture(sampler2D(skin_texture, tex_sampler), uv).r;
    // 反转一下数值
    // tex_gray = (1.0 - tex_gray);
    
    float alpha = aastep(tex_gray, stroke_mask);
    // float alpha = step(stroke_mask,tex_gray);

    vec4 stroke_color = vec4(vec3(1.0), alpha);

    if (show_outline) {
        vec4 outline_color = outline;
        outline_color.a = aastep(tex_gray, outline_mask);
        stroke_color = lerp(outline_color, stroke_color, alpha);
    }
    if (show_shadow) {
        vec4 shadow_color = vec4(0.2, 0.2, 0.2, 1.0);
        shadow_color.a = aastep(tex_gray, outline_mask);
        stroke_color = lerp(shadow_color, stroke_color, alpha);
    }
    frag_color = stroke_color;
}
