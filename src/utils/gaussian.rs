// 高斯分布函数：
// 描述了在给定平均值（通常以希腊字母μ表示） 和标准差（以希腊字母σ表示）下的概率分布情
//
//
// 标准差是离均差平方和平均后的方根，具体的计算方法是：先求所有成绩减去平均成绩后的
// 平方，再对所得的值求平均值（方差），最后把平均值开根号，就得到这组数据的标准差
//
// 标准差的本质是这样的：给定一个种群，68%的个体数据分布在距平均值1个标准差的范围 内，
// 98%的个体数据分布在2个标准差的范围之内，99.7%的个体分布在3个标准差的范围之内。

use std::f32::consts::PI;

// 标准高斯函数
// 标准正态分布是位置参数 μ=0，尺度参数σ^2 = 1的正态分布
pub fn standard(x: f32) -> f32 {
    let p = 1.0 / (2.0 * PI).sqrt();
    p * (-x.powf(2.0) / 2.0).exp()
}

pub fn gaussian(x: f32, miu: f32, sigma: f32) -> f32 {
    let p = 1.0 / (sigma * (2.0 * PI).sqrt());
    p * (-(x - miu).powf(2.0) / (2.0 * sigma.powf(2.0)))
}

pub fn lookup_table() -> [f32; 32] {
    let mut table = [0.0; 32];
    let first = standard(0.0);
    // 将值正规化到 0.95 ～ 0
    let scale = 0.95 / first;
    table[0] = first * scale;

    let s = 3.0 / 16.0;
    for i in 1..15 {
        table[i] = standard(s * i as f32) * scale;
    }

    table
}
