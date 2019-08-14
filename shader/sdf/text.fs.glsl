layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 1) uniform texture2D skin_texture;
layout(set = 0, binding = 2) uniform sampler tex_sampler;

// 反走样
float aastep(float value)
{
    // float afwidth = length(vec2(dFdx(value), dFdy(value))) * 0.70710678118654757;
    float afwidth = (1.0 / 32.0) * (1.4142135623730951 / (2.0 * gl_FragCoord.w));
    return smoothstep(0.5 - afwidth, 0.5 + afwidth, value);
}

void main(void)
{
    vec4 tex_color = texture(sampler2D(skin_texture, tex_sampler), uv);
    float alpha = aastep(1.0 - tex_color.r);
    frag_color = vec4(vec3(1.0), alpha);
    // frag_color = tex_color;
}
