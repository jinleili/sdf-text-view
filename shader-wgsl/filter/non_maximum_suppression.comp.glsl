layout(local_size_x = 16, local_size_y = 16) in;

layout(set = 0, binding = 0) uniform InfoUniform
{
    // info[0] = pic.width ;
    // info[1] = pic.height;
    // info[2] = 0: direction x, 1: direction y
    ivec4 info;
    // threshold[0] = lowerThreshold
    // threshold[1] = upperThreshold
    vec4 threshold;
};
layout(binding = 1, rgba8) uniform image2D input_pic;
layout(binding = 2, r32f) uniform image2D output_pic;

void main()
{
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    if (uv.x > (info.x - 1) || uv.y > (info.y - 1)) {
        return;
    }
    vec3 currentGradientAndDirection = imageLoad(input_pic, uv).rgb;
    ivec2 gradientDirection = ivec2(((currentGradientAndDirection.gb * 2.0) - 1.0) * info.xy);
    float firstSampledGradientMagnitude = imageLoad(input_pic, uv + gradientDirection).r;
    float secondSampledGradientMagnitude = imageLoad(input_pic, uv - gradientDirection).r;
    float multiplier = step(firstSampledGradientMagnitude, currentGradientAndDirection.r);
    multiplier = multiplier * step(secondSampledGradientMagnitude, currentGradientAndDirection.r);

    float thresholdCompliance = smoothstep(threshold[0], threshold[1], currentGradientAndDirection.r);
    multiplier = multiplier * thresholdCompliance;
    
    imageStore(output_pic, uv, vec4(vec3(multiplier), 1.0));
}