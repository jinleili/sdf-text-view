// D2Q9 流体相关的定义及函数

layout(set = 0, binding = 0) uniform FluidUniform
{
    // e 表示D2Q9离散速度模型速度空间的速度配置
    // w 表示每个方向上的权重
    vec4 e_and_w[9];
    // lattice 在正规化坐标空间的大小
    vec2 lattice_size;
    ivec2 lattice_num;
};

// rgb 表示对应 lattice 上的宏观速度密度
struct FluidCell {
    vec4 color;
};

// Cs 表示声速
const float Cs2 = 1.0 / 3.0;
// 流体特征
const float physCharLength = 1.0;
const float physCharVelocity = 1.0;
const float physViscosity = 0.01;
const float latticeCharVelocity = 0.01;

/// Emergent fluid numbers
// 雷诺数 = UL/v
const float Re = 220.0;
// 入流速度
const float uLB = 0.05; 
// 粘度 viscosity
const float nuLB = uLB / Re;
const float tau = 3.0 * nuLB + 0.5;
//弛豫时间
const float relaxationFrequency = 1.0 / tau;

// 为了获得比较好的模拟效果，马赫数需要确保比较小
// Ma = U/Cs， U
// 为宏观流速，为了减小误差，特征流速需要设定成比较小的值，最好不要超过 0.1
const float Ma = latticeCharVelocity * sqrt(Cs2);
const float Kn = Ma / Re;

// 获取离散速度模型速度空间的速度配置
vec2 e(int direction)
{
    return e_and_w[direction].xy;
}

// 获取某个方向上的权重
float w(int direction)
{
    return e_and_w[direction].z;
}

// 平衡分布函数
float equilibrium(vec2 velocity, float rho, int direction, float usqr)
{
    // D2Q9包分布公式
    float e_dot_u = dot(e(direction), velocity);

    // pow(x, y) 要求 x 参数不能为负，e_dot_u
    // 是有可能为负的，所以求它的平方不能用  pow 内置函数 当 i == 0 时，feq =
    // rho * weight[i] * (1.0 - 1.5 * dot(velocity, velocity) / Cs2);
    // return rho * w(i) * (1.0 + 3.0 * e_dot_u / Cs2 + 4.5 * (e_dot_u * e_dot_u) / pow(Cs2, 2.0) - 1.5 * dot(velocity, velocity) / Cs2);
    return rho * w(direction) * (1.0 + 3.0 * e_dot_u + 4.5 * (e_dot_u * e_dot_u) - usqr);
}

// 格子的索引
int indexOfLattice(ivec2 uv)
{
    return (uv.x + (uv.y * lattice_num.x) * 9);
}

int indexOfFluid(ivec2 uv)
{
    return uv.x + (uv.y * lattice_num.x);
}

bool isBounceBackCell(int material) {
    return material == 2;
}

bool isBulkFluidCell(int material)
{
    return material == 1 || material == 5 || material == 6;
}

// 流入区
bool isInflowCell(int material)
{
    return material == 5;
}

// 流出区
bool isOutflowCell(int material)
{
    return material == 6;
}