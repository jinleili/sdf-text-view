pub static touch_points: [[f32; 2]; 50] = [
    [121.33332824707031, 358.3333282470703],
    [111.66665649414062, 363.0],
    [108.0, 368.6666564941406],
    [102.66665649414062, 376.0],
    [96.66665649414062, 385.3333282470703],
    [90.66665649414062, 396.3333282470703],
    [84.66665649414062, 407.6666564941406],
    [78.0, 420.0],
    [71.33332824707031, 432.6666564941406],
    [64.33332824707031, 446.6666564941406],
    [60.0, 460.3333282470703],
    [56.33332824707031, 471.6666564941406],
    [54.0, 482.6666564941406],
    [53.0, 490.6666564941406],
    [53.0, 496.0],
    [53.0, 498.6666564941406],
    [54.33332824707031, 499.3333282470703],
    [60.33332824707031, 499.6666564941406],
    [70.33332824707031, 499.6666564941406],
    [84.66665649414062, 495.3333282470703],
    [104.33332824707031, 487.3333282470703],
    [126.0, 478.6666564941406],
    [150.0, 470.3333282470703],
    [178.0, 460.6666564941406],
    [203.66665649414062, 452.3333282470703],
    [228.3333282470703, 445.6666564941406],
    [249.3333282470703, 440.6666564941406],
    [266.6666564941406, 437.6666564941406],
    [282.3333282470703, 436.3333282470703],
    [295.0, 436.0],
    [304.3333282470703, 436.0],
    [310.3333282470703, 436.0],
    [313.6666564941406, 437.3333282470703],
    [316.3333282470703, 440.0],
    [317.3333282470703, 442.3333282470703],
    [318.3333282470703, 444.6666564941406],
    [318.3333282470703, 446.3333282470703],
    [318.3333282470703, 449.3333282470703],
    [318.3333282470703, 453.6666564941406],
    [318.3333282470703, 459.6666564941406],
    [318.3333282470703, 466.3333282470703],
    [318.3333282470703, 473.6666564941406],
    [318.3333282470703, 481.3333282470703],
    [317.3333282470703, 489.0],
    [314.6666564941406, 498.0],
    [309.6666564941406, 511.6666564941406],
    [299.6666564941406, 533.6666564941406],
    [285.6666564941406, 563.6666564941406],
    [271.0, 596.3333282470703],
    [258.6666564941406, 612.0],
];

static touch_intervals: [f32; 50] = [
    0.0,
    0.01672494411468506,
    0.017393946647644043,
    0.016311049461364746,
    0.016270041465759277,
    0.0172499418258667,
    0.01630699634552002,
    0.017235994338989258,
    0.01620006561279297,
    0.015674948692321777,
    0.01689302921295166,
    0.017540931701660156,
    0.016053080558776855,
    0.01746201515197754,
    0.016197919845581055,
    0.017525076866149902,
    0.01630091667175293,
    0.016080021858215332,
    0.017646074295043945,
    0.016000986099243164,
    0.01721501350402832,
    0.01547694206237793,
    0.017171025276184082,
    0.017297029495239258,
    0.01618194580078125,
    0.017367959022521973,
    0.016251087188720703,
    0.016412973403930664,
    0.01787400245666504,
    0.016047954559326172,
    0.015812993049621582,
    0.01738905906677246,
    0.016330957412719727,
    0.016224980354309082,
    0.017332077026367188,
    0.016389012336730957,
    0.01746690273284912,
    0.01601409912109375,
    0.0160830020904541,
    0.017663002014160156,
    0.015883922576904297,
    0.016787052154541016,
    0.016932010650634766,
    0.016344904899597168,
    0.017204999923706055,
    0.016355037689208984,
    0.016314983367919922,
    0.01720607280731201,
    0.01666390895843506,
    0.008861064910888672,
];

use crate::math::Position;
use crate::vertex::{Pos, PosBrush};
use nalgebra_glm as glm;

static PI_2: f32 = std::f32::consts::PI / 2.0;
static TWO_PI: f32 = std::f32::consts::PI * 2.0;

pub fn cal_curve() -> (Vec<PosBrush>, Vec<u16>) {
    let mut vertice: Vec<PosBrush> = vec![];
    let mut indices: Vec<u16> = vec![];

    let first_point = touch_points.first();
    for i in 1..52 {
        let cur = touch_points[i];
        let interval = touch_intervals[i];
    }

    (vertice, indices)
}

// pub fn generate_vertices(width: u32) -> (Vec<PosBrush>, Vec<u16>) {
//     let mut vertices: Vec<PosBrush> = vec![];
//     let mut indices: Vec<u16> = vec![];

//     let mut half_w = (width as f32) / 2.0;
//     // 线段边上用来实施抗锯齿的的像素
//     let mut blur_pixel = 1.0;
//     if half_w >= 2.0 {
//         blur_pixel = 2.0;
//     }

//     half_w += blur_pixel;

//     let mut last = touch_points.first().unwrap();
//     let last_slope = slope_ridian(&last, &touch_points[1]);
//     let mut pos = cal_pos(last, last_slope, last_slope, half_w, true);
//     // pos[1].weight = 3.0;
//     vertices.append(&mut pos.to_vec());
//     // vertices.push(PosWeight::new([last.x, last.y, 0.0], 0.0));
//     for i in 1 .. (touch_points.len() - 1) {
//         let cur = touch_points[i];
//         let slope = slope_ridian(last, &cur);
//         let mut blend = slope;
//         // atan2 求出的θ取值范围是[-PI, PI]
//         // 角度直接融合采用算中间值的办法可能会导致计算出的线宽坐标点方向反转
//         // 1,2 或者 3，4 象限之内的角度，可以相加之后求平均
//         if i < (touch_points.len() - 1) {
//             let next = slope_ridian(&cur, &touch_points[i + 1]);
//             if (slope >= 0.0 && next >= 0.0) || (slope < 0.0 && next < 0.0) {
//                 blend = (slope + next) / 2.0;
//             } else {
//                 blend = slope + (TWO_PI - (slope - next).abs()) / 2.0;
//                 if blend > std::f32::consts::PI {
//                     blend -= TWO_PI;
//                 } else if blend < -std::f32::consts::PI {
//                     blend += TWO_PI;
//                 }
//             }
//         }

//         let pos = cal_pos(&cur, slope, blend, half_w, false);
//         vertices.append(&mut pos.to_vec());
//         // 重复一遍顶点做为下一个纹理的起点
//         let pos = cal_pos(&cur, slope, blend, half_w, true);
//         vertices.append(&mut pos.to_vec());

//         let cur_index0 = (i * 6 - 3) as u16;
//         let last_index0 = cur_index0 - 3_u16;

//         indices.append(&mut vec![
//             last_index0,
//             last_index0 + 1,
//             cur_index0,
//             cur_index0,
//             cur_index0 + 1,
//             last_index0 + 1,
//             cur_index0 + 1,
//             last_index0 + 1,
//             last_index0 + 2,
//             cur_index0 + 1,
//             last_index0 + 2,
//             cur_index0 + 2,
//         ]);

//         last = &touch_points[i];
//     }
//     (vertices, indices)
// }

pub fn get_touchpoints() -> Vec<PosBrush> {
    let mut vertices: Vec<PosBrush> = vec![];
    for i in 0..touch_points.len() {
        let p = touch_points[i];
        vertices.push(PosBrush::new([p[0], p[1], 0.0], [0.0, 0.0], [1.0, 0.0, 0.0]))
    }
    vertices
}

pub fn generate_vertices(width: u32) -> (Vec<PosBrush>, Vec<u16>, Vec<PosBrush>) {
    let mut vertices: Vec<PosBrush> = vec![];
    let mut indices: Vec<u16> = vec![];
    let mut control_vertices: Vec<PosBrush> = vec![];

    // z 值一样的情况下，过绘制产生的结果异常
    let mut z = 0.0;
    let z_offset = 0.001;

    let (curve_points, control_points) = generate_curve_points();

    let mut last = curve_points.first().unwrap();
    let last_slope = slope_ridian(&last, &curve_points[1]);
    let pos = cal_pos2(last, last_slope, z);
    vertices.append(&mut pos.to_vec());
    indices.append(&mut vec![0, 1, 2, 0, 2, 3]);

    // for i in 1 .. curve_points.len() {
    //     z += z_offset;

    //     let cur = curve_points[i];
    //     let slope = slope_ridian(last, &cur);
    //     let mut blend = slope;
    //     // atan2 求出的θ取值范围是[-PI, PI]
    //     // 角度直接融合采用算中间值的办法可能会导致计算出的线宽坐标点方向反转
    //     // 1,2 或者 3，4 象限之内的角度，可以相加之后求平均
    //     if i < (curve_points.len() - 1) {
    //         let next = slope_ridian(&cur, &curve_points[i + 1]);
    //         if (slope >= 0.0 && next >= 0.0) || (slope < 0.0 && next < 0.0) {
    //             blend = (slope + next) / 2.0;
    //         } else {
    //             blend = slope + (TWO_PI - (slope - next).abs()) / 2.0;
    //             if blend > std::f32::consts::PI {
    //                 blend -= TWO_PI;
    //             } else if blend < -std::f32::consts::PI {
    //                 blend += TWO_PI;
    //             }
    //         }
    //     }

    //     let pos = cal_pos2(&cur, blend, z);
    //     vertices.append(&mut pos.to_vec());

    //     let cur_index0 = (i * 4) as u16;
    //     indices.append(&mut vec![
    //         cur_index0,
    //         cur_index0 + 1,
    //         cur_index0 + 2,
    //         cur_index0,
    //         cur_index0 + 2,
    //         cur_index0 + 3,
    //     ]);

    //     last = &curve_points[i];
    // }
    for i in 0..control_points.len() {
        let p = control_points[i];
        control_vertices.push(PosBrush::new([p.x, p.y, 0.0], [0.0, 0.0], [0.0, 1.0, 1.0]))
    }
    (vertices, indices, control_vertices)
}

fn generate_curve_points() -> (Vec<Position>, Vec<Position>) {
    let mut points: Vec<Position> = vec![];
    let mut control_points: Vec<Position> = vec![];

    let mut pre: Position = touch_points.first().unwrap().into();
    points.push(pre);

    let mut pre_ctrl_point = Position::zero();
    let mut pre_angle = 0.0;

    for i in 1..=(touch_points.len() - 2) {
        let cur: Position = touch_points[i].into();
        let d = pre.distance(&cur);

        let next: Position = touch_points[i + 1].into();
        // cur 垂直映射到 [pre, next] 上，得到 q,
        // [pre, q] 的中点就是控制点在 [pre, next] 直线上的垂直映射点 p
        // 算出 p 到 cur 的偏移 offset, pre + offset 就是要找的控制点 c0
        let mut norm: glm::TVec2<f32> = next.minus(&pre).into_vec2();
        norm = glm::normalize(&norm);
        let dot: f32 = glm::dot(&cur.minus(&pre).into_vec2(), &norm).into();
        let ridian = next.slope_ridian(&pre);

        // 两个矢量间的夹角, 计算公式：A.B = |A||B|cos(a)
        let intersection_angle = (dot / (d * next.distance(&pre))).acos().abs();

        if d < 26.8 && intersection_angle < 3.2 {
            // 两点之间距离小于 1.5 且跟上下两点的方向变化不大， 就不做插值
            points.push(cur);
            pre_ctrl_point = Position::zero();
        } else {
            let mut q: Position = Position::new(dot * ridian.cos(), dot * ridian.sin());
            let p = q.divide_f(2.0).add(&pre);
            let offset = cur.minus(&p);
            let c0 = pre.add(&offset);

            let step = (d / 1.1).floor() + 1.0 + ((pre_angle + intersection_angle) / 0.25).floor();
            let mut curve_points = if pre_ctrl_point.is_equal_zero() {
                super::curve::Curve::quadratic_points(pre, c0, cur, 1, step as u32)
            } else {
                super::curve::Curve::cubic_points(pre, pre_ctrl_point, c0, cur, 1, step as u32)
            };

            points.append(&mut curve_points);
            control_points.push(c0);
            pre_ctrl_point = c0;
        }
        pre = cur;
        pre_angle = intersection_angle;
    }
    (points, control_points)
}

pub fn slope_ridian(first: &Position, last: &Position) -> f32 {
    // atan2 求出的θ取值范围是[-PI, PI]
    let radian = (first.y - last.y).atan2(first.x - last.x);
    radian
}

// uv_lp = true 表示坐标为 0,0 ~ 0, 1,
pub fn cal_pos(
    cur: &[f32; 2], original: f32, blend_radian: f32, half_w: f32, uv_lb: bool,
) -> [PosBrush; 3] {
    let radian = -PI_2 + blend_radian;
    let other = -std::f32::consts::PI + radian;
    let mut p0 = PosBrush::new(
        [cur[0] + half_w * radian.cos(), cur[1] + half_w * radian.sin(), 0.0],
        [if uv_lb { 0.4 } else { 0.6 }, 0.0],
        [half_w, 0.0, 0.0],
    );
    let mut p1 = PosBrush::new(
        [cur[0] + half_w * other.cos(), cur[1] + half_w * other.sin(), 0.0],
        [if uv_lb { 0.4 } else { 0.6 }, 1.0],
        [half_w, 0.0, 0.0],
    );
    [
        p0,
        PosBrush::new([cur[0], cur[1], 0.0], [if uv_lb { 0.4 } else { 0.6 }, 0.5], [0.0, 0.0, 0.0]),
        p1,
    ]
}

pub fn cal_pos2(cur: &Position, slope: f32, z: f32) -> [PosBrush; 4] {
    let w = 32.0;
    // 原点为中心的四个坐标点
    let lb = glm::TVec4::new(-w, -w, z, 1.0);
    let lt = glm::TVec4::new(-w, w, z, 1.0);
    let rt = glm::TVec4::new(w, w, z, 1.0);
    let rb = glm::TVec4::new(w, -w, z, 1.0);

    // 默认纹理是朝向左边的，需要旋转 - PI 来匹配斜率
    let mut matrix = glm::TMat4::identity();
    let radian: glm::TVec1<f32> = glm::TVec1::new(slope - std::f32::consts::PI);
    matrix = glm::rotate(&matrix, radian[0], &glm::vec3(0.0, 0.0, 1.0));

    [
        PosBrush::new(cal_vertex(&lb, &matrix, cur), [0.0, 0.0], [0.0, 0.0, 0.0]),
        PosBrush::new(cal_vertex(&lt, &matrix, cur), [0.0, 1.0], [0.0, 0.0, 0.0]),
        PosBrush::new(cal_vertex(&rt, &matrix, cur), [1.0, 1.0], [0.0, 0.0, 0.0]),
        PosBrush::new(cal_vertex(&rb, &matrix, cur), [1.0, 0.0], [0.0, 0.0, 0.0]),
    ]
}

fn cal_vertex(p: &glm::TVec4<f32>, matrix: &glm::TMat4<f32>, offset: &Position) -> [f32; 3] {
    let v: [f32; 4] = (matrix * p).into();
    [v[0] + offset.x, v[1] + offset.y, v[2]]
}
