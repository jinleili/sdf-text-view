layout(local_size_x = 16, local_size_y = 16) in;

layout(set = 0, binding = 0) uniform InfoUniform
{
    // info[0] = pic.width ;
    // info[1] = pic.height;
    ivec4 info;
};
layout(binding = 1, r8) uniform image2D input_pic;
layout(binding = 2, r8) uniform image2D output_pic;

void main()
{
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    if (uv.x < 1 || uv.x > (info.x - 2) || uv.y < 1 || uv.y > (info.y - 2)) {
        return;
    }
    float bottomLeftIntensity = imageLoad(input_pic, ivec2(uv.x - 1, uv.y + 1)).r;
    float topRightIntensity = imageLoad(input_pic, ivec2(uv.x + 1, uv.y - 1)).r;
    float topLeftIntensity = imageLoad(input_pic, ivec2(uv.x - 1, uv.y - 1)).r;
    float bottomRightIntensity = imageLoad(input_pic, ivec2(uv.x + 1, uv.y + 1)).r;
    float leftIntensity = imageLoad(input_pic, ivec2(uv.x - 1, uv.y)).r;
    float rightIntensity = imageLoad(input_pic, ivec2(uv.x + 1, uv.y)).r;
    float bottomIntensity = imageLoad(input_pic, ivec2(uv.x, uv.y + 1)).r;
    float topIntensity = imageLoad(input_pic, ivec2(uv.x, uv.y - 1)).r;
    float centerIntensity = imageLoad(input_pic, uv).r;

    float pixelIntensitySum = bottomLeftIntensity + topRightIntensity + topLeftIntensity + bottomRightIntensity + leftIntensity + rightIntensity + bottomIntensity + topIntensity + centerIntensity;
    float sumTest = step(1.5, pixelIntensitySum);
    float pixelTest = step(0.01, centerIntensity);

    imageStore(output_pic, uv, vec4(vec3(sumTest * pixelTest), 1.0));
}