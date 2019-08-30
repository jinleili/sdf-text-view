layout(local_size_x = 1, local_size_y = 16) in;

#include sdf_layout_and_fn

void main()
{
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    if (uv.y > (info.y - 1)) {
        return;
    }
    
    // transform along rows
    sdf1d(uv.y * info.x, 1, info.x, uv.y * (info.x + 1), 1);
}