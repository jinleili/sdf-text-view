
layout(binding = 0, rgba8) uniform image2D inout_texture;

void main() {
	ivec2 tex_uv = ivec2(gl_GlobalInvocationID.xy);
    vec4 color = imageLoad(inout_texture, tex_uv);

	vec4 res =  vec4(vec3((color.r + color.g + color.b) / 3.0), color.a);
	imageStore(inout_texture, tex_uv, res);
}