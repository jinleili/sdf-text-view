use std::borrow::BorrowMut;
use std::f32::consts::PI;
use std::rc::Rc;
use std::vec::Vec;

use crate::math::Position;
use crate::vertex::{Pos, PosBrush};
use nalgebra_glm as glm;

static PI_2: f32 = PI / 2.0;
static TWO_PI: f32 = PI * 2.0;

#[derive(PartialEq)]
pub enum TouchState {
    Normal = 0,
    Start = 1,
    Moving = 2,
    End = 3,
}

pub struct StrokeCalculator {
    touch_points: Vec<Rc<Vec<Position>>>,
    cur_touch: Vec<Position>,
    // 当前正在写的 path
    cur_path: Vec<Position>,
    cur_touch_state: TouchState,
    // 上一个点的斜率
    last_slope: f32,
    //
    last_z: f32,
    z_offset: f32,
    // 间隔几个触摸点再开始计算绘制
    draw_start_gap: usize,
}

impl StrokeCalculator {
    pub fn new() -> Self {
        StrokeCalculator {
            touch_points: vec![],
            cur_touch: vec![],
            cur_path: vec![],
            cur_touch_state: TouchState::Normal,
            last_slope: 0.0,
            last_z: 0.0,
            z_offset: 0.001,
            draw_start_gap: 2,
        }
    }

    pub fn path_start(&mut self, p: [f32; 2]) -> (Vec<PosBrush>, Vec<u16>, Vec<PosBrush>) {
        let pos: Position = p.into();
        // 保存从 touch_start 到 touch_end 的 point
        self.cur_touch = vec![pos];

        (vec![], vec![], vec![])
    }

    // 起笔运算: 有尖峰，斜切
    fn execute_path_start(&mut self) -> (Vec<PosBrush>, Vec<u16>, Vec<PosBrush>) {
        let mut vertices: Vec<PosBrush> = vec![];
        let mut indices: Vec<u16> = vec![];
        let control_vertices: Vec<PosBrush> = vec![];

        let pos: &Position = self.cur_touch.first().unwrap();

        // 起笔斜率,
        // 由于使用的 mvp 左上角为圆点，so ,沿 x 轴向下旋转为正, 所以需要反转一下旋转角度
        self.last_slope = -(PI * 0.33);
        self.last_slope = TWO_PI - self.last_slope;
        self.cur_path = vec![];
        // 起笔自动在位置前后运动一定距离
        let mut index_offset = 0;
        for i in -5..=(5) {
            let p = pos.new_by_slope_n_dis(self.last_slope, ((i * -1) as f32) * 1.5);
            let brushs = cal_pos(&p, self.last_slope, self.last_z);
            vertices.append(&mut brushs.to_vec());
            indices.append(&mut vec![
                index_offset + 0,
                index_offset + 1,
                index_offset + 2,
                index_offset + 0,
                index_offset + 2,
                index_offset + 3,
            ]);
            self.last_z += self.z_offset;
            index_offset += 4;
        }
        // 将自动运动到的结束点也当做一个 touch 点，与接下来的点生成自然平滑过渡
        let end_p = pos.new_by_slope_n_dis(self.last_slope, -5.0 * 1.5);
        self.cur_path.push(pos.clone());
        self.cur_path.push(end_p);

        // self.touch_points.push(self.cur_touch.clone());

        (vertices, indices, control_vertices)
    }

    pub fn path_move_linear(&mut self, p: [f32; 2]) -> (Vec<PosBrush>, Vec<u16>, Vec<PosBrush>) {
        self.cur_touch.push(p.into());

        if self.cur_touch_state == TouchState::Normal {
            if self.cur_touch.len() <= self.draw_start_gap {
                return (vec![], vec![], vec![]);
            } else {
                self.cur_touch_state = TouchState::Moving;
                return self.execute_path_start();
            }
        }

        let index = self.cur_touch.len() - self.draw_start_gap;
        let pos: Position = self.cur_touch[index];
        self.cur_path.push(pos.clone());
        let mut vertices: Vec<PosBrush> = vec![];
        let mut indices: Vec<u16> = vec![];
        let control_vertices: Vec<PosBrush> = vec![];

        self.last_z += self.z_offset;
        let last = self.cur_path[self.cur_path.len() - 2];

        let mut curve_points = generate_linear_points(&self.cur_path);
        for i in 0..curve_points.len() {
            self.last_z += self.z_offset;

            let cur = curve_points[i];
            let mut slope = self.last_slope;

            // atan2 求出的θ取值范围是[-PI, PI]
            // 角度直接融合采用算中间值的办法可能会导致计算出的线宽坐标点方向反转
            // 1,2 或者 3，4 象限之内的角度，可以相加之后求平均

            // 转换一个方向需要 5 帧以上
            let mut slope_to = cur.slope_ridian(&last);

            if slope > PI && slope_to < PI {
                slope_to = slope + (TWO_PI - (slope_to - slope).abs()) / 5.0;
            } else if slope < PI && slope_to > PI {
                slope_to = slope + (slope_to + slope.abs()) / 5.0;
            } else {
                slope_to = slope + (slope_to - slope) / 5.0;
            }
            if slope_to > TWO_PI {
                slope_to -= TWO_PI;
            } else if slope_to < -TWO_PI {
                slope_to += TWO_PI;
            }
            self.last_slope = slope_to;
            // println!("cal slope: {}, {}", slope, slope_to);

            let brushs = cal_pos(&cur, slope_to, self.last_z);
            vertices.append(&mut brushs.to_vec());
            let index_offset = (i * 4) as u16;
            indices.append(&mut vec![
                index_offset,
                index_offset + 1,
                index_offset + 2,
                index_offset,
                index_offset + 2,
                index_offset + 3,
            ]);
        }
        let pre = self.cur_path[&self.cur_path.len() - 2];
        let cur = self.cur_path[&self.cur_path.len() - 1];
        let d = pre.distance(&cur);
        println!("distance: {}", d);

        (vertices, indices, control_vertices)
    }

    pub fn path_end(&mut self, p: [f32; 2]) -> (Vec<PosBrush>, Vec<u16>, Vec<PosBrush>) {
        let mut vertices: Vec<PosBrush> = vec![];
        let mut indices: Vec<u16> = vec![];
        let control_vertices: Vec<PosBrush> = vec![];
        let pos = p.into();
        self.cur_path.push(pos);
        self.cur_touch.push(pos);

        let is_sharp_end = crate::brush::is_sharp_end(&self.cur_touch);
        println!("is_sharp_end: {}", is_sharp_end);

        (vertices, indices, control_vertices)
    }
}

fn cal_pos(cur: &Position, slope: f32, z: f32) -> [PosBrush; 4] {
    let w = 82.0;
    // 原点为中心的四个坐标点
    let lb = glm::TVec4::new(-w, -w, z, 1.0);
    let lt = glm::TVec4::new(-w, w, z, 1.0);
    let rt = glm::TVec4::new(w, w, z, 1.0);
    let rb = glm::TVec4::new(w, -w, z, 1.0);

    // 默认纹理是朝向左边的，(需要旋转 - PI 来匹配斜率?), 0 slope 时，纹理正好是向左
    let mut new_slope = slope;
    // if new_slope > PI {
    //     new_slope -= PI;
    // } else {
    //     new_slope += PI;
    // }
    let mut matrix = glm::TMat4::identity();
    // let radian: glm::TVec1<f32> = glm::TVec1::new(new_slope);
    // matrix = glm::rotate(&matrix, radian[0], &glm::vec3(0.0, 0.0, 1.0));

    [
        PosBrush::new(cal_vertex(&lb, &matrix, cur), [0.0, 0.0], [0.0, 0.0, 0.0]),
        PosBrush::new(cal_vertex(&lt, &matrix, cur), [0.0, 1.0], [0.0, 0.0, 0.0]),
        PosBrush::new(cal_vertex(&rt, &matrix, cur), [1.0, 1.0], [0.0, 0.0, 0.0]),
        PosBrush::new(cal_vertex(&rb, &matrix, cur), [1.0, 0.0], [0.0, 0.0, 0.0]),
        // PosBrush::new(cal_vertex(&lb, &matrix, cur), [0.25, 0.25], [0.0, 0.0, 0.0]),
        // PosBrush::new(cal_vertex(&lt, &matrix, cur), [0.25, 0.75], [0.0, 0.0, 0.0]),
        // PosBrush::new(cal_vertex(&rt, &matrix, cur), [0.75, 0.75], [0.0, 0.0, 0.0]),
        // PosBrush::new(cal_vertex(&rb, &matrix, cur), [0.75, 25.0], [0.0, 0.0, 0.0]),
    ]
}

fn cal_vertex(p: &glm::TVec4<f32>, matrix: &glm::TMat4<f32>, offset: &Position) -> [f32; 3] {
    let v: [f32; 4] = (matrix * p).into();
    [v[0] + offset.x, v[1] + offset.y, v[2]]
}

// 拿最后 3 个点计算 pre -> cur 的插值
fn generate_curve_points(points: &Vec<Position>) -> (Vec<Position>, Vec<Position>) {
    let mut curve_points: Vec<Position> = vec![];
    let mut control_points: Vec<Position> = vec![];

    let pre = points[points.len() - 3];
    let cur = points[points.len() - 2];
    let next = points[points.len() - 1];

    let d = pre.distance(&cur);

    // cur 垂直映射到 [pre, next] 上，得到 q,
    // [pre, q] 的中点就是控制点在 [pre, next] 直线上的垂直映射点 p
    // 算出 p 到 cur 的偏移 offset, pre + offset 就是要找的控制点 c0
    let mut norm: glm::TVec2<f32> = next.minus(&pre).into_vec2();
    norm = glm::normalize(&norm);
    let dot: f32 = glm::dot(&cur.minus(&pre).into_vec2(), &norm).into();
    let ridian = next.slope_ridian(&pre);

    // 两个矢量间的夹角, 计算公式：A.B = |A||B|cos(a)
    let intersection_angle = (dot / (d * next.distance(&pre))).acos().abs();

    if d < 1.6 && intersection_angle < 0.4 {
        // 两点之间距离小于 1.5 且跟上下两点的方向变化不大， 就不做插值
        curve_points.push(cur);
    } else {
        let mut q: Position = Position::new(dot * ridian.cos(), dot * ridian.sin());
        let p = q.divide_f(2.0).add(&pre);
        let offset = cur.minus(&p);
        let c0 = pre.add(&offset);

        let step = (d / 1.1).floor() + 1.0 + (intersection_angle / 0.25).floor();
        curve_points = super::curve::Curve::quadratic_points(pre, c0, cur, 1, step as u32);

        control_points.push(c0);
    }

    (curve_points, control_points)
}

// 线性插值
fn generate_linear_points(points: &Vec<Position>) -> Vec<Position> {
    let mut linear_points: Vec<Position> = vec![];
    let pre = points[points.len() - 2];
    let cur = points[points.len() - 1];
    let d = pre.distance(&cur);
    // 两点之间距离小于 1.5， 就不做插值
    if d > 3.6 {
        let count = (d / 3.6).ceil();
        let step_d = 1.0 / count;
        for i in 1..(count as i32) {
            let step = step_d * i as f32;
            let p = Position {
                x: pre.x * (1.0 - step) + cur.x * step,
                y: pre.y * (1.0 - step) + cur.y * step,
            };
            linear_points.push(p);
        }
    }
    linear_points.push(cur);

    linear_points
}
