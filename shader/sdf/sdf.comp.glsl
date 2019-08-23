layout(local_size_x = 1, local_size_y = 1) in;

layout(set = 0, binding = 0) uniform InfoUniform
{
    // info[0] = pic.width ; [1] = pic.height;
    // [2] = 0 | 1 (iter by y, x),
    // [2] = 2 (save final distance to input_pic's r channel)
    // [3] = 0 | 1 ( front or background distance fields)
    ivec4 info;
};
layout(binding = 1, r8) uniform image2D input_pic;
// g_front = front distance fields, g_background = background distance fields
// Cannot reuse block name within the same shader
layout(set = 0, binding = 2) buffer EDTFront { float g_front[]; };
layout(set = 0, binding = 3) buffer EDTBackground { float g_background[]; };
layout(set = 0, binding = 4) buffer EDTFn { float f[]; };
layout(set = 0, binding = 5) buffer EDTTempDistance { float d[]; };
layout(set = 0, binding = 6) buffer EDTTempV { int v[]; };
layout(set = 0, binding = 7) buffer EDTTempZ { float z[]; };

const float INF = 99999.0;
const float OUTLINE = 0.25;

int pixel_index(ivec2 uv)
{
    return uv.y * info[0] + uv.x;
}

float reset_f(int x, int y) {
    if (info[3] == 0) {
        return g_front[pixel_index(ivec2(x, y))].r;
    } else {
        return g_background[pixel_index(ivec2(x, y))].r;
    }
}

void reset_v_n_z()
{
    int max_length = info.x > info.y ? info.x : info.y;
    for (int i = 0; i < max_length; i++) {
        v[i] = 0;
        z[i] = 0.0;
    }
    z[max_length] = 0.0;
    z[0] = -INF;
    z[1] = INF;
}

void update_sdf(int x, int y, float val) {
    if (info[3] == 0) {
        g_front[pixel_index(ivec2(x, y))] = val;
    } else {
        g_background[pixel_index(ivec2(x, y))] = val;
    }
}

void sdf1d(int length)
{
    int k = 0;
    int r = 0;
    float s = 0.0;

    reset_v_n_z();

    for (int q = 1; q < length; q++) {
        do {
            r = v[k];
            s = (f[q] + float(q * q) - f[r] - float(r * r)) / float(2 * (q - r));
            // 实际情况: k 不会小于 0
            k--;
        } while (s <= z[k]);
        k++;
        v[k] = q;
        z[k] = s;
        z[k + 1] = INF;
    }

    k = 0;
    for (int q = 0; q < length; q++) {
        while (z[k + 1] < float(q)) {
            k++;
        }
        r = v[k];
        d[q] = f[r] + float(pow(q - r, 2));
    }
}

void main() {
    if (info[2] == 0) {
        // transform along columns
        int x = int(gl_GlobalInvocationID.x);
        for (int y = 0; y < info.y; y++) {
            f[y] = reset_f(x, y);
        }

        sdf1d(info.y);

        for (int y = 0; y < info.y; y++) {
            update_sdf(x, y, d[y]);
        }
    } else if (info[2] == 1) {
        // transform along rows
        int y = int(gl_GlobalInvocationID.y);
        for (int x = 0; y < info.x; x++) {
            f[x] = reset_f(x, y);
        }

        sdf1d(info.x);

        for (int x = 0; y < info.x; x++) {
            update_sdf(x, y, d[x]);
        }
    } else {
        // output final distans fields
        ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
        float dis = sqrt(g_background[pixel_index(uv)]) - sqrt(g_front[pixel_index(uv)]);
        float luma = (1.0 - (dis / 8.0 + OUTLINE));

        float final = clamp(luma, 0.0, 1.0);
        imageStore(input_pic, uv, vec4(final, 0.0, 0.0, 0.0));
    }
}