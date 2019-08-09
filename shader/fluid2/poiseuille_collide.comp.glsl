layout(local_size_x = 1, local_size_y = 1) in;

#include fluid_layout_and_fn

// layout (set = 0, binding = 1, r32f) uniform image2D CollideBuffer;
// layout (set = 0, binding = 2, r32f) uniform image2D StreamBuffer;
layout(set = 0, binding = 1) buffer CollideBuffer { float collideCells[]; };
layout(set = 0, binding = 2) buffer StreamBuffer { float streamCells[]; };
layout(set = 0, binding = 3) buffer FluidBuffer { FluidCell fluidCells[]; };

// 回弹方向对应的传播索引
const int bounceBackDirection[9] = int[](0, 3, 4, 1, 2, 7, 8, 5, 6);

const vec2 force = vec2(0.01, 0.0);

// 更新流体速度等信息
void updateFluid(ivec2 uv, vec2 velocity, float rho)
{
    if (isinf(velocity.x) || isinf(velocity.y) || isinf(rho)) {
        return;
    }
    int destIndex = uv.x + uv.y * lattice_num.x;
    // fluidCells[destIndex].color = float[](velocity.x, velocity.y, rho);
    fluidCells[destIndex].color[0] = velocity.x;
    fluidCells[destIndex].color[1] = velocity.y;
    fluidCells[destIndex].color[2] = rho;
}

// collide
void updateCollide(ivec2 uv, int direction, float collide)
{
    // 避免出现 NaN， inf, -inf, 将值限定在一个范围
    collideCells[indexOfLattice(uv) + direction] = clamp(collide, -10000.0, 10000);
}

void main()
{
    ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    // 用来判断当前是不是边界，障碍等
    // Material number meaning (geometry is only changed by the interaction shader)
    int material = int(fluidCells[indexOfFluid(uv)].color[3]);

    float f_i[9];
    for (int i = 0; i < 9; i++) {
        // f_i[i] = streamCells[indexOfLattice(uv) + i];
        f_i[i] = streamCells[indexOfLattice(uv) + i];
    }

    //格子点的迁移过程
    vec2 velocity = vec2(0.0);
    float rho = 0.0;

    for (int i = 0; i < 9; i++) {
        rho += f_i[i];
        // U = sum_fi*ei / rho
        velocity += e(i) * f_i[i];
    }
    velocity = velocity / rho;

    if (isBulkFluidCell(material)) {
        if (isInflowCell(material)) {
            velocity = vec2(0.005, 0.0);
        }
        // if (isOutflowCell(material)) {
        //     rho = 1.0;
        // }
        updateFluid(uv, velocity, rho);

        // 平衡方程最后一项：1.5 * 速度绝对值的平方
        float usqr = 1.5 * (velocity.x * velocity.x + velocity.y * velocity.y);
        for (int i = 0; i < 9; i++) {
            // 碰撞
            // float collide = f_i[i] + relaxationFrequency * (equilibrium(velocity, rho, i, usqr) - f_i[i]);
            float collide = f_i[i] - relaxationFrequency * (f_i[i] - equilibrium(velocity, rho, i, usqr));

            // 体力驱动
            // collide = collide[i] + Gamma[i] * (dot(e[i], force));
            updateCollide(uv, i, collide); 
        }
    }
    if (isBounceBackCell(material)) {
        for (int i = 0; i < 9; i++) {
            // 回弹
            float collide = f_i[bounceBackDirection[i]];

            updateCollide(uv, i, collide);
        }
    }
    
}