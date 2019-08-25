layout(local_size_x = 1, local_size_y = 1) in;

layout(set = 0, binding = 0) uniform InfoUniform
{
    // info[0] = pic.width ;
    // info[1] = pic.height;
    // info[2] = 0 | 1 (iter by y, x), = 1 (init edt), = 2 (save final distance to input_pic's r channel)
    // info[3] = 0 | 1 ( front or background distance fields)
    ivec4 info;
};
layout(binding = 1, r8) uniform image2D input_pic;
// g_front = front distance fields, g_background = background distance fields
// Cannot reuse block name within the same shader
layout(set = 0, binding = 2) buffer EDTFront { float g_front[]; };
layout(set = 0, binding = 3) buffer EDTBackground { float g_background[]; };
layout(set = 0, binding = 4) buffer EDTFn { float f[]; };
layout(set = 0, binding = 5) buffer EDTTempV { int v[]; };
layout(set = 0, binding = 6) buffer EDTTempZ { float z[]; };

const float INF = 99999.0;

int pixel_coord(int x, int y)
{
    return y * info.x + x;
}

int pixel_index(ivec2 uv)
{
    return uv.y * info.x + uv.x;
}

void reset_f(int index)
{
    if (info[3] == 0) {
        f[index] = g_front[index];
    } else {
        f[index] = g_background[index];
    }
}

void update_sdf(int index, float val)
{
    if (info[3] == 0) {
        g_front[index] = val;
    } else {
        g_background[index] = val;
    }
}

void sdf1d(int offset, int stride, int len)
{
    // restore temp arr to default value
    for (int q = 0; q < len; q++) {
        int real_index = offset + q * stride;
        reset_f(real_index);
        v[real_index] = 0;
        z[real_index] = 0.0;
    }
    // z[offset + (len - 1) * stride] = 0.0;
    z[offset] = -INF;
    z[offset + stride] = INF;

    int k = 0;
    int r = 0;
    // 1D r value, like the q
    int r1d;
    float s = 0.0;

    // 1D squared distance transform
    for (int q = 1; q < len; q++) {
        int real_q = offset + q * stride;
        do {
            r = v[offset + k * stride];
            // 1D r value, like the q
            r1d = int((r - offset) / stride);
            s = (f[real_q] + float(q * q) - f[r] - float(r1d * r1d)) / float(2 * (q - r1d));
            // 实际情况: k 不会小于 0
        } while (s <= z[offset + k * stride] && --k > (-1));
        k++;
        v[offset + k * stride] = real_q;
        z[offset + k * stride] = s;
        z[offset + (k + 1) * stride] = INF;
    }

    k = 0;
    for (int q = 0; q < len; q++) {
        while (z[offset + (k + 1) * stride] < float(q)) {
            k++;
        }
        r = v[offset + k * stride];
        r1d = int((r - offset) / stride);
        update_sdf(offset + q * stride, f[r] + float((q - r1d) * (q - r1d)));
    }
}

void main()
{
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);

    if (info[2] == 2) {
        // init front && background distance fields
        float luma = imageLoad(input_pic, uv).r;
        if (luma > 0.949) {
            g_front[pixel_index(uv)] = INF;
            g_background[pixel_index(uv)] = 0.0;
        } else if (luma < 0.01) {
            g_front[pixel_index(uv)] = 0.0;
            g_background[pixel_index(uv)] = INF;
        } else {
            g_front[pixel_index(uv)] = pow(max(0.0, luma - 0.5), 2.0);
            g_background[pixel_index(uv)] = pow(max(0.0, 0.5 - luma), 2.0);
        }
    } else if (info[2] == 0) {
        // transform along columns
        sdf1d(uv.x, info.x, info.y);
    } else if (info[2] == 1) {
        // transform along rows
        sdf1d(uv.y * info.x, 1, info.x);
    } else {
        // output final distans fields
        float dis = sqrt(g_background[pixel_index(uv)]) - sqrt(g_front[pixel_index(uv)]);
        float luma = (1.0 - (dis / 8.0 + 0.25));

        float final = clamp(luma, 0.0, 1.0);
        // float final = clamp(sqrt(g_background[pixel_index(uv)]), 0.0, 1.0);
        imageStore(input_pic, uv, vec4(final, 0.0, 0.0, 0.0));
    }
}