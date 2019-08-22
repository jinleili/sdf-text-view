layout(local_size_x = 1, local_size_y = 1) in;

layout(set = 0, binding = 0) uniform InfoUniform
{
    // info[0] = pic.width ; [1] = pic.height; [2] = 0 | 1 (iter by y, x)
    ivec3 info;
};
layout(binding = 1, rgba8) uniform image2D input_pic;
// r = front distance fields, g = background distance fields
layout(binding = 2, rg32f) uniform image2D grid;

const float INF = 99999.0;

int pixel_index(x, y) {
    return y * info[0] + x;
}

void update_grid(int x, int y, int channel, float val) {
    grid[pixel_index(x, y)][channel] = val;
}

void main() {
    if (info[2] == 0) {
        ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
        float luma = imageLoad(input_pic, uv).r;
        if (luma >= 0.95) {
            update_grid(uv.x, uv.y, 0, INF);
            update_grid(uv.x, uv.y, 1, 0.0);
        } else if (luma < 0.01) {
            update_grid(uv.x, uv.y, 0, 0.0);
            update_grid(uv.x, uv.y, 1, INF);
        } else {
            update_grid(uv.x, uv.y, 0, max(0.0, luma - 0.5).powf(2.0));
            update_grid(uv.x, uv.y, 1, max(0.0, 0.5 - luma).powf(2.0));
        }
    } else if (info[2] == 1) {
        // transform along columns
        int x = gl_GlobalInvocationID.x;
        float[] f
        for (int y=0; y<info.y; y++) {

        }
    }
}