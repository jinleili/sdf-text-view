layout(local_size_x = 16, local_size_y = 16) in;

layout(set = 0, binding = 0) uniform InfoUniform
{
    // info[0] = pic.width ;
    // info[1] = pic.height;
    ivec4 info;
};
layout(binding = 1, r32f) uniform image2D input_pic;
layout(binding = 2, rgba8) uniform image2D output_pic;

void main()
{
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    if (uv.x > (info.x - 1) || uv.y > (info.y - 1)) {
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
    // float bottomLeftIntensity = imageLoad(input_pic, ivec2(uv.x - 1, uv.y - 1)).r;
    // float topRightIntensity = imageLoad(input_pic, ivec2(uv.x + 1, uv.y + 1)).r;
    // float topLeftIntensity = imageLoad(input_pic, ivec2(uv.x - 1, uv.y + 1)).r;
    // float bottomRightIntensity = imageLoad(input_pic, ivec2(uv.x + 1, uv.y - 1)).r;
    // float leftIntensity = imageLoad(input_pic, ivec2(uv.x - 1, uv.y)).r;
    // float rightIntensity = imageLoad(input_pic, ivec2(uv.x + 1, uv.y)).r;
    // float bottomIntensity = imageLoad(input_pic, ivec2(uv.x, uv.y - 1)).r;
    // float topIntensity = imageLoad(input_pic, ivec2(uv.x, uv.y + 1)).r;

    vec2 gradientDirection;
    gradientDirection.x = -bottomLeftIntensity - 2.0 * leftIntensity - topLeftIntensity + bottomRightIntensity + 2.0 * rightIntensity + topRightIntensity;
    gradientDirection.y = -topLeftIntensity - 2.0 * topIntensity - topRightIntensity + bottomLeftIntensity + 2.0 * bottomIntensity + bottomRightIntensity;
    float gradientMagnitude = length(gradientDirection);
    vec2 normalizedDirection = normalize(gradientDirection);
    // Offset by 1-sin(pi/8) to set to 0 if near axis, 1 if away
    normalizedDirection = sign(normalizedDirection) * floor(abs(normalizedDirection) + 0.617316);
    // Place -1.0 - 1.0 within 0 - 1.0
    normalizedDirection = (normalizedDirection + 1.0) * 0.5;

    imageStore(output_pic, uv, vec4(gradientMagnitude, normalizedDirection.x, normalizedDirection.y, 1.0));
}