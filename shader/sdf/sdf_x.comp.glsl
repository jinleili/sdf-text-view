layout(local_size_x = 16, local_size_y = 1) in;

#include sdf/sdf_layout_and_fn.glsl

void main()
{
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    if (uv.x > (info.x - 1)) {
        return;
    }

    // transform along columns
    sdf1d(uv.x, info.x, info.y, uv.x, info.x + 1);
}