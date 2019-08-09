use crate::math::Position;
use std::vec::Vec;

// 笔画端点计算

// 收笔：藏锋，露峰（基于最后几个点的 distance 趋势）
pub fn is_sharp_end(points: &Vec<Position>) -> bool {
    let len = points.len();
    // 小于 5 个点的笔画只能是藏锋
    if len < 6 {
        return false;
    }
    let mut list: Vec<f32> = vec![];
    for i in (len - 6)..len {
        let pre = points[i - 1];
        let cur = points[i];
        list.push(pre.distance(&cur));
    }
    let mut count = 0;
    let mut base = list.first().unwrap();
    for i in 1..list.len() {
        let cur = &list[i];
        if cur >= base {
            count += 1;
        } else {
            count -= 1;
        }
        base = cur;
    }
    if count >= 0 {
        return true;
    } else {
        return false;
    }
}
