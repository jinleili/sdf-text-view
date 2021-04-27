layout(local_size_x = 16, local_size_y = 16) in;

layout(set = 0, binding = 0) uniform InfoUniform
{
    // info[0] = pic.width ;
    // info[1] = pic.height;
    ivec4 info;
};
layout(binding = 1, rgba8) uniform image2D input_pic;
layout(binding = 2, r32f) uniform image2D out_pic;

// Values from "Graphics Shaders: Theory and Practice" by Bailey and Cunningham
const highp vec3 W = vec3(0.2125, 0.7154, 0.0721);

int get_pixel_index(int x, int y)
{
    return y * info.x + x;
}

int get_pixel_index(ivec2 uv)
{
    return uv.y * info.x + uv.x;
}

void main() {
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    if (uv.x > (info.x - 1) || uv.y > (info.y - 1)) {
        return;
    }
    vec4 texture_color = imageLoad(input_pic, uv);
    float luminance = dot(texture_color.rgb, W);
    // save to single channel gray image
    imageStore(out_pic, uv, vec4(vec3(luminance), 1.0));
}