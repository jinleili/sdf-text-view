// 快速高斯模糊
// 1，分解二维计算为两次一维计算来大幅减小计算规模；
// 2，基于 GPU 纹理采样的特点来优化；
// 3，缩小纹理分辨率-> blur -> 放大到原始尺寸；
// https://software.intel.com/en-us/blogs/2014/07/15/an-investigation-of-fast-real-time-gpu-based-image-blur-algorithms
// we can take advantage of fixed function GPU hardware, namely samplers,
// which can load two (in our case) neighboring pixel values and return an interpolated result based on the
// provided texture coordinate values, all for approximately the cost of one texture read.

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 1) uniform BlurUniform {
    float uv_step;
    // 实施模糊的坐标轴方向
    float direction_x;
};
layout(set = 0, binding = 2) uniform texture2D skin_texture;
layout(set = 0, binding = 3) uniform sampler tex_sampler;

// ES 2.0 不支持常量数组
const float weight[5] = float[](0.13, 0.115, 0.11, 0.105, 0.10 );

void main(void) {
	// sampler2D tex = sampler2D(skin_texture, tex_sampler);
	vec4 temp_color = texture(sampler2D(skin_texture, tex_sampler), uv) * weight[0];
	bool is_direction_x = direction_x >= 1.0 ? true : false;
	for (int i = 1; i < 5; i++) {
	    float factor = float(i) * uv_step;
	    vec2 offset_uv = vec2(is_direction_x ? factor : 0., is_direction_x ? 0. : factor);
	    temp_color += texture(sampler2D(skin_texture, tex_sampler), uv + offset_uv ) * weight[i];
	    temp_color += texture(sampler2D(skin_texture, tex_sampler), uv - offset_uv ) * weight[i];
	}
	frag_color = vec4(temp_color.rgb, 1.);
   
}

