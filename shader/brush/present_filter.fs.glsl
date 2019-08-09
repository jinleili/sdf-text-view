layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 1) uniform texture2D skin_texture;
layout(set = 0, binding = 2) uniform sampler tex_sampler;

void main(void) {
	vec4 temp = texture(sampler2D(skin_texture, tex_sampler), uv);
    // 颜色不够深的地方转成透明，避免糊的感觉
    // if (temp.b < 0.3) {
    //     temp.a = 0.0;
    // }
    frag_color = temp;
}
