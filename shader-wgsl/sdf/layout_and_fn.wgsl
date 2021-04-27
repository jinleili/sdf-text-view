
[[block]]
struct InfoUniform {
    // info[0] = pic.width ;
    // info[1] = pic.height;
    // info[2] = 0 | 1 (iter by y, x), = 1 (init edt), = 2 (save final distance to input_pic's r channel)
    // info[3] = 0 | 1 ( front or background distance fields)
     info: vec4<i32>;
     padding: [[stride(16)]] array<vec4<i32>, 15>;
};

[[block]]
struct StoreFloat {
    data: [[stride(4)]] array<f32>;
};

[[block]]
struct StoreInt {
    data: [[stride(4)]] array<i32>;
};

[[group(1), binding(0)]] var<uniform> params: InfoUniform;
// g_front = front distance fields, g_background = background distance fields
// Cannot reuse block name within the same shader
[[group(0), binding(0)]] var<storage> g_front: [[access(read_write)]] StoreFloat;
[[group(0), binding(1)]] var<storage> g_background: [[access(read_write)]] StoreFloat;
[[group(0), binding(2)]] var<storage> v: [[access(read_write)]] StoreInt;
[[group(0), binding(3)]] var<storage> z: [[access(read_write)]] StoreFloat;

[[group(0), binding(4)]] var input_pic: [[access(read)]] texture_storage_2d<r32float>;
[[group(0), binding(5)]] var output_pic: [[access(write)]] texture_storage_2d<r32float>;

let INF: f32 = 1.0E10;

fn get_pixel_index(x: i32, y: i32) -> i32 {
    return y * params.info.x + x;
}

fn get_pixel_index2(uv: vec2<i32>) -> i32 {
    return uv.y * params.info.x + uv.x;
}

fn get_f(index: i32) -> f32 {
    return 0.0;
    // if (params.info.w == 1) {
    //     return g_background.data[u32(index)];
    // } else {
    //     return g_front.data[u32(index)];
    // }
}

fn update_sdf(index: i32, val: f32) {
    if (params.info[3] == 1) {
        g_background.data[u32(index)] = val;
    } else {
        g_front.data[u32(index)] = val;
    }
}

fn sdf1d(offset: i32, stride: i32, len: i32, offset_z: i32, stride_z: i32) {
    // reset temp array to default value
    for (var q: i32 = 0; q < len; q = q + 1) {
        v.data[offset + q * stride] = 0;
        // z buffer's size equals to input_pic's (size.width + 1, size.height + 1)
        z.data[offset_z + q * stride_z] = 0.0;
    }
    z.data[offset_z + len * stride_z] = 0.0;
    z.data[offset_z] = -INF;
    z.data[offset_z + stride_z] = INF;

    var k: i32 = 0;
    var r: i32 = 0;
    var s: f32 = 0.0;

    // 1D squared distance transform
    for (var q: i32 = 0; q < len; q = q + 1) {
        let pixel_index_q: i32 = offset + q * stride;
        let f0: f32 = get_f(pixel_index_q) + f32(q * q);
        loop {
            r = v.data[offset + k * stride];
            // porabola q and k intersect at s
            s = (f0 - get_f(r * stride + offset) - f32(r * r)) / f32(2 * (q - r));
            // since z.data[0] = -INF, k will not less than 0
            if (s <= z.data[offset_z + k * stride_z] && --k > (-1)) {
                continue;
            } else {
                break;
            }
        };
        k = k + 1;
        // when v.data[k] equeals zero,the pixel index is not zero, always need use (q * stride + offset) calculate get_f()'s parameter,
        // so, cannot save pixel_index_q into v.data[k].
        v.data[offset + k * stride] = q;
        z.data[offset_z + k * stride_z] = s;
        z.data[offset_z + (k + 1) * stride_z] = INF;
    }

    k = 0;
    for (var q: i32 = 0; q < len; q = q + 1) {
        loop {
            if (z.data[offset_z + (k + 1) * stride_z] >= f32(q)) {
                break;
            }
            k = k + 1;
        }
        r = v.data[offset + k * stride];
        update_sdf(offset + q * stride, get_f(r * stride + offset) + f32((q - r) * (q - r)));
    }
}