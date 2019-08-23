layout(local_size_x = 1, local_size_y = 1) in;

layout(set = 0, binding = 0) uniform InfoUniform
{
    // info[0] = pic.width ; [1] = pic.height;
    // [2] = 0 | 1 (iter by y, x), [3] = 0 | 1 ( front or background distance fields)
    ivec4 info;
};

layout(binding = 1, rgba8) uniform image2D input_pic;
// g_front = front distance fields, g_background = background distance fields
// Cannot reuse block name within the same shader
layout(set = 0, binding = 2) buffer EDTFront { float g_front[]; };
layout(set = 0, binding = 3) buffer EDTBackground { float g_background[]; };

const float INF = 99999.0;

int pixel_index(ivec2 uv)
{
    return uv.y * info[0] + uv.x;
}

void main()
{
    // init front && background distance fields
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    float luma = imageLoad(input_pic, uv).r;
    if (luma >= 0.95) {
        g_front[pixel_index(uv)] = INF;
        g_background[pixel_index(uv)] = 0.0;
    } else if (luma < 0.01) {
        g_front[pixel_index(uv)] = 0.0;
        g_background[pixel_index(uv)] = INF;
    } else {
        g_front[pixel_index(uv)] = pow(max(0.0, luma - 0.5), 2.0);
        g_background[pixel_index(uv)] = pow(max(0.0, 0.5 - luma), 2.0);
    }
}