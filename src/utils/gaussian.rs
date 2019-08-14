// 高斯分布函数：
// 描述了在给定平均值（通常以希腊字母μ表示） 和标准差（以希腊字母σ表示）下的概率分布情
use std::f32::consts::PI;

// 标准高斯函数
// 标准正态分布是位置参数 μ=0，尺度参数σ^2 = 1的正态分布
#[allow(dead_code)]
pub fn standard(x: f32) -> f32 {
    let p = 1.0 / (2.0 * PI).sqrt();
    p * (-x.powf(2.0) / 2.0).exp()
}

#[allow(dead_code)]
pub fn gaussian(x: f32, miu: f32, sigma: f32) -> f32 {
    let p = 1.0 / (sigma * (2.0 * PI).sqrt());
    p * (-(x - miu).powf(2.0) / (2.0 * sigma.powf(2.0)))
}

#[allow(dead_code)]
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
