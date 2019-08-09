layout(local_size_x = 1, local_size_y = 1) in;

#include fluid_layout_and_fn

layout(set = 0, binding = 1) buffer CollideBuffer
{
    float collideCells[];
};
layout(set = 0, binding = 2) buffer StreamBuffer { float streamCells[]; };
layout(set = 0, binding = 3) buffer FluidBuffer { FluidCell fluidCells[]; };

//弛豫时间
const float dRevTau = 1.0 / 0.75;

// 回弹方向对应的传播索引
const int bounceBackDirection[9] = int[](0, 3, 4, 1, 2, 7, 8, 5, 6);

// 普通回弹
// direction: 方向索引
void setNormalBounceBack(ivec2 uv, int direction)
{
    streamCells[indexOfLattice(uv) + direction] = collideCells[indexOfLattice(uv) + bounceBackDirection[direction]];
}

// 流入（迁移）：将周围点的量迁移到当前点，参考 coursera 上tktk
void propagate(ivec2 uv, int direction)
{
    // streaming 是流入，colliding 是流出
    ivec2 new_uv = uv + ivec2(e(direction));
    // 当前方向的量是周围格子上反方向上的量流入过来的
    streamCells[indexOfLattice(uv) + direction] = collideCells[indexOfLattice(new_uv) + bounceBackDirection[direction]];
}

void main()
{
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    // 用来判断当前是不是边界，障碍等
    // Material number meaning (geometry is only changed by the interaction shader)
    int material = int(fluidCells[indexOfFluid(uv)].color[3]);

    // 四周边界
    if (isBounceBackCell(material)) {
        // imageLoad 坐标是左上角为（0， 0），右下角为（w, h)
        // 回弹方向的变量，应该到 src 上当前位置的 lattice 上的反方向的值
        // for (int i = 0; i < 9; i++) {
        //     setNormalBounceBack(uv, i);
        // }
        for (int i = 0; i < 9; i++) {
            propagate(uv, i);
        }
        // 左边界
        // if (uv.x == 0) {
        //     setNormalBounceBack(uv, 1);
        //     setNormalBounceBack(uv, 5);
        //     setNormalBounceBack(uv, 8);
        //     propagate(uv, 0);
        //     propagate(uv, 2);
        //     propagate(uv, 3);
        //     propagate(uv, 4);
        //     propagate(uv, 6);
        //     propagate(uv, 7);

        // } else if (uv.x == lattice_num.x - 1) {
        //     // 右边界
        //     setNormalBounceBack(uv, 3);
        //     setNormalBounceBack(uv, 6);
        //     setNormalBounceBack(uv, 7);
        //     propagate(uv, 0);
        //     propagate(uv, 2);
        //     propagate(uv, 1);
        //     propagate(uv, 4);
        //     propagate(uv, 5);
        //     propagate(uv, 8);
        // } else if (uv.y == 0) {
        //     // 顶边界
        //     setNormalBounceBack(uv, 4);
        //     setNormalBounceBack(uv, 8);
        //     setNormalBounceBack(uv, 7);
        //     propagate(uv, 0);
        //     propagate(uv, 2);
        //     propagate(uv, 1);
        //     propagate(uv, 3);
        //     propagate(uv, 5);
        //     propagate(uv, 6);
        // } else {
        //     // 顶边界
        //     setNormalBounceBack(uv, 2);
        //     setNormalBounceBack(uv, 5);
        //     setNormalBounceBack(uv, 6);
        //     propagate(uv, 0);
        //     propagate(uv, 3);
        //     propagate(uv, 1);
        //     propagate(uv, 4);
        //     propagate(uv, 7);
        //     propagate(uv, 8);
        // }
    } else {
        for (int i = 0; i < 9; i++) {
            propagate(uv, i);
        }
    }
}