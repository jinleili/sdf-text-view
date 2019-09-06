// 快速高斯模糊
// 1，分解二维计算为两次一维计算来大幅减小计算规模；
// 2，基于 GPU 纹理采样的特点来优化；
// 3，缩小纹理分辨率-> blur -> 放大到原始尺寸；
// https://software.intel.com/en-us/blogs/2014/07/15/an-investigation-of-fast-real-time-gpu-based-image-blur-algorithms
// we can take advantage of fixed function GPU hardware, namely samplers,
// which can load two (in our case) neighboring pixel values and return an interpolated result based on the
// provided texture coordinate values, all for approximately the cost of one texture read.

layout(local_size_x = 16, local_size_y = 16) in;

layout(set = 0, binding = 0) uniform InfoUniform
{
    // info[0] = pic.width ;
    // info[1] = pic.height;
    // info[2] = 0: direction x, 1: direction y
    ivec4 info;
};
layout(binding = 1, r8) uniform image2D input_pic;
layout(binding = 2, r8) uniform image2D output_pic;

const float weight[3] = { 0.39894346935609776, 0.2959625730773051, 0.004565692244646007 };

void main()
{
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    if (uv.x > (info.x - 1) || uv.y > (info.y - 1)) {
        return;
    }
    bool is_direction_x = info[2] == 0 ? true : false;
    float temp = imageLoad(input_pic, uv).r * weight[0];
    for (int i = 1; i < 3; i++) {
        ivec2 offset_uv = ivec2(is_direction_x ? 1 : 0, is_direction_x ? 0 : 1);
        temp += imageLoad(input_pic, uv + offset_uv).r * weight[i];
        temp += imageLoad(input_pic, uv - offset_uv).r * weight[i];
    }

    imageStore(output_pic, uv, vec4(vec3(temp), 1.0));
}