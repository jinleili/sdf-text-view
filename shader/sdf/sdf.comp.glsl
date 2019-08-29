layout(local_size_x = 16, local_size_y = 16) in;

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
    if (info[3] == 0) {
        return g_front[index];
    } else {
        return g_background[index];
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

void sdf1d(int offset, int stride, int len, int offset_z, int stride_z)
{
    // restore temp array to default value
    for (int q = 0; q < len; q++) {
        v[offset + q * stride] = 0;
        // z buffer's size equals to input_pic's (size.width + 1, size.height + 1)
        z[offset_z + q * stride_z] = 0.0;
    }
    z[offset_z + len * stride_z] = 0.0;
    z[offset_z] = -INF;
    z[offset_z + stride_z] = INF;


    int k = 0;
    int pixel_index_r = 0;
    // 1D r value, like the q
    int r1d;
    float s = 0.0;

    // 1D squared distance transform
    for (int q = 1; q < len; q++) {
        int pixel_index_q = offset + q * stride;
        do {
            pixel_index_r = v[offset + k * stride];
            r1d = (pixel_index_r - offset) / stride;
            // porabola q and k intersect at s
            s = (get_f(pixel_index_q) + float(q * q) - get_f(pixel_index_r) - float(r1d * r1d)) / float(2 * (q - r1d));
            // since z[0] = -INF, k will not less than 0
        } while (s <= z[offset_z + k * stride_z] && --k > (-1));
        k += 1;
        v[offset + k * stride] = pixel_index_q;
        z[offset_z + k * stride_z] = s;
        z[offset_z + (k + 1) * stride_z] = INF;
    }

    k = 0;
    for (int q = 0; q < len; q++) {
        while (z[offset_z + (k + 1) * stride_z] < float(q)) {
            k += 1;
        }
        pixel_index_r = v[offset + k * stride];
        r1d = (pixel_index_r - offset) / stride;
        update_sdf(offset + q * stride, get_f(pixel_index_r) + float((q - r1d) * (q - r1d)));
        // update_sdf(pixel_index_r, INF);
    }
}

void main()
{
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    if (uv.x > (info.x -1) || uv.y > (info.y -1)) {
        return;
    }
    
    int pixel_index = get_pixel_index(uv);
    // z[pixel_index] = float(uv.x);

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
    } else if (info[2] == 0) {
        // transform along columns
        sdf1d(uv.x, info.x, info.y, uv.x, info.x + 1);
    } else if (info[2] == 1) {
        // transform along rows
        sdf1d(uv.y * info.x, 1, info.x, uv.y * (info.x + 1), 1);
    } else {
        // output final distans fields
        float dis = sqrt(g_background[pixel_index]) - sqrt(g_front[pixel_index]);
        // float dis = sqrt(g_background[pixel_index]);
        float luma = clamp((1.0 - (dis / 8.0 + 0.25)), 0.0, 1.0);
        // float luma = clamp(dis, 0.0, 1.0);

        imageStore(input_pic, uv, vec4(luma, 0.0, 0.0, 0.0));
    }
}