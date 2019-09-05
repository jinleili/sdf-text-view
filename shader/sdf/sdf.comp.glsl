layout(local_size_x = 16, local_size_y = 16) in;

#include sdf/sdf_layout_and_fn.glsl

void main()
{
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    if (uv.x > (info.x -1) || uv.y > (info.y -1)) {
        return;
    }

    int pixel_index = get_pixel_index(uv);

    if (info[2] == 2) {
        // init front && background distance fields
        float luma = 1.0 - imageLoad(input_pic, uv).r;
        if (luma > 0.949) {
            g_front[pixel_index] = INF;
            g_background[pixel_index] = 0.0;
        } else if (luma < 0.1) {
            g_front[pixel_index] = 0.0;
            g_background[pixel_index] = INF;
        } else {
            g_front[pixel_index] = pow(max(0.0, luma - 0.5), 2.0);
            g_background[pixel_index] = pow(max(0.0, 0.5 - luma), 2.0);
        }
    } else {
        // output final distans fields
        float dis = sqrt(g_background[pixel_index]) - sqrt(g_front[pixel_index]);
        float luma = clamp((1.0 - (dis / 22.0 + 0.25)), 0.0, 1.0);

        imageStore(input_pic, uv, vec4(luma, 0.0, 0.0, 0.0));
    }
}