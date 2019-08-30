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
layout(set = 0, binding = 4) buffer EDTTempV { int v[]; };
layout(set = 0, binding = 5) buffer EDTTempZ { float z[]; };

const float INF = 1.0E10;

int get_pixel_index(int x, int y)
{
    return y * info.x + x;
}

int get_pixel_index(ivec2 uv)
{
    return uv.y * info.x + uv.x;
}

float get_f(int index)
{
    if (info[3] == 1) {
        return g_background[index];
    } else {
        return g_front[index];
    }
}

void update_sdf(int index, float val)
{
    if (info[3] == 1) {
        g_background[index] = val;
    } else {
        g_front[index] = val;
    }
}

void sdf1d(int offset, int stride, int len, int offset_z, int stride_z)
{
    // reset temp array to default value
    for (int q = 0; q < len; q++) {
        v[offset + q * stride] = 0;
        // z buffer's size equals to input_pic's (size.width + 1, size.height + 1)
        z[offset_z + q * stride_z] = 0.0;
    }
    z[offset_z + len * stride_z] = 0.0;
    z[offset_z] = -INF;
    z[offset_z + stride_z] = INF;

    int k = 0;
    int r = 0;
    float s = 0.0;

    // 1D squared distance transform
    for (int q = 1; q < len; q++) {
        int pixel_index_q = offset + q * stride;
        do {
            r = v[offset + k * stride];
            // porabola q and k intersect at s
            s = (get_f(pixel_index_q) + float(q * q) - get_f(r * stride + offset) - float(r * r)) / float(2 * (q - r));
            // since z[0] = -INF, k will not less than 0
        } while (s <= z[offset_z + k * stride_z] && --k > (-1));
        k += 1;
        // when v[k] equeals zero,the pixel index is not zero, always need use (q * stride + offset) calculate get_f()'s parameter,
        // so, cannot save pixel_index_q into v[k].
        v[offset + k * stride] = q;
        z[offset_z + k * stride_z] = s;
        z[offset_z + (k + 1) * stride_z] = INF;
    }

    k = 0;
    for (int q = 0; q < len; q++) {
        while (z[offset_z + (k + 1) * stride_z] < float(q)) {
            k += 1;
        }
        r = v[offset + k * stride];
        update_sdf(offset + q * stride, get_f(r * stride + offset) + float((q - r) * (q - r)));
    }
}
