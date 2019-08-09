layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 1) uniform texture2D skin_texture;
layout(set = 0, binding = 2) uniform sampler tex_sampler;

void main(void) {
	frag_color = texture(sampler2D(skin_texture, tex_sampler), uv);
	// frag_color = vec4(0.1, 0.2, 0.3, 1.0);
}

