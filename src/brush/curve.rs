use crate::math::Position;
use crate::vertex::{Pos, PosBrush, PosWeight};
use nalgebra_glm as glm;

static PI_2: f32 = std::f32::consts::PI / 2.0;
static TWO_PI: f32 = std::f32::consts::PI * 2.0;

pub struct Curve {
    pub points: Vec<Position>,
    pub edge_distance: f32,
    pub blur_distance: f32,
}

impl Curve {
    // 二次曲线
    pub fn quadratic(start: Position, ctrl: Position, end: Position) -> Self {
        let d = distance(&vec![start, ctrl, end]);
        let mut stride_count = (d / 10.0).floor() as u32;
        if stride_count < 3 {
            stride_count = 3;
        }
        let mut points: Vec<Position> =
            super::curve::Curve::quadratic_points(start, ctrl, end, 0, stride_count);

        Curve { points, edge_distance: 0.5, blur_distance: 0.1 }
    }
    pub fn quadratic_points(
        start: Position, ctrl: Position, end: Position, iter_start: u32, stride_count: u32,
    ) -> Vec<Position> {
        let stride = 1.0 / (stride_count - 1) as f32;
        let mut points: Vec<Position> = vec![];
        for i in iter_start..stride_count {
            let t = stride * i as f32;
            let t2 = t.powf(2.0);
            let t12 = (1.0 - t).powf(2.0);
            let p = Position::new(
                start.x * t12 + ctrl.x * 2.0 * (1.0 - t) * t + end.x * t2,
                start.y * t12 + ctrl.y * 2.0 * (1.0 - t) * t + end.y * t2,
            );
            points.push(p);
        }
        points
    }

    // 三次曲线
    pub fn cubic(start: Position, ctrl0: Position, ctrl1: Position, end: Position) -> Self {
        let d = distance(&vec![start, ctrl0, ctrl1, end]);
        let mut stride_count = (d / 10.0).floor() as u32;
        if stride_count < 4 {
            stride_count = 4;
        }
        let points: Vec<Position> =
            super::curve::Curve::cubic_points(start, ctrl0, ctrl1, end, 0, stride_count);

        Curve { points, edge_distance: 0.5, blur_distance: 0.1 }
    }

    pub fn cubic_points(
        start: Position, ctrl0: Position, ctrl1: Position, end: Position, iter_start: u32,
        stride_count: u32,
    ) -> Vec<Position> {
        let stride = 1.0 / (stride_count - 1) as f32;
        let mut points: Vec<Position> = vec![];
        for i in iter_start..stride_count {
            let t = stride * i as f32;
            let t2 = t.powf(2.0);
            let t3 = t.powf(3.0);
            let t12 = (1.0 - t).powf(2.0);
            let t13 = (1.0 - t).powf(3.0);
            let p = Position::new(
                start.x * t13
                    + ctrl0.x * 3.0 * t12 * t
                    + ctrl1.x * 3.0 * (1.0 - t) * t2
                    + end.x * t3,
                start.y * t13
                    + ctrl0.y * 3.0 * t12 * t
                    + ctrl1.y * 3.0 * (1.0 - t) * t2
                    + end.y * t3,
            );
            points.push(p);
        }

        points
    }

    pub fn generate_vertices(
        &mut self, width: u32, index_offset: u16,
    ) -> (Vec<PosBrush>, Vec<u16>) {
        let mut vertices: Vec<PosBrush> = vec![];
        let mut indices: Vec<u16> = vec![];

        let mut half_w = (width as f32) / 2.0;
        // 线段边上用来实施抗锯齿的的像素
        let mut blur_pixel = 1.0;
        if half_w >= 2.0 {
            blur_pixel = 2.0;
        }
        self.edge_distance = half_w;
        self.blur_distance = blur_pixel;
        half_w += blur_pixel;

        let mut last = self.points.first().unwrap();
        let last_slope = last.slope_ridian(&self.points[1]);
        let mut pos = cal_pos(last, last_slope, last_slope, half_w);
        // pos[1].weight = 3.0;
        vertices.append(&mut pos.to_vec());
        // vertices.push(PosWeight::new([last.x, last.y, 0.0], 0.0));
        for i in 1..self.points.len() {
            let cur = self.points[i];
            let slope = last.slope_ridian(&cur);
            let mut blend = slope;
            // atan2 求出的θ取值范围是[-PI, PI]
            // 角度直接融合采用算中间值的办法可能会导致计算出的线宽坐标点方向反转
            // 1,2 或者 3，4 象限之内的角度，可以相加之后求平均
            if i < (self.points.len() - 1) {
                let next = cur.slope_ridian(&self.points[i + 1]);
                if (slope >= 0.0 && next >= 0.0) || (slope < 0.0 && next < 0.0) {
                    blend = (slope + next) / 2.0;
                } else {
                    blend = slope + (TWO_PI - (slope - next).abs()) / 2.0;
                    if blend > std::f32::consts::PI {
                        blend -= TWO_PI;
                    } else if blend < -std::f32::consts::PI {
                        blend += TWO_PI;
                    }
                }
            }

            let pos = cal_pos(&cur, slope, blend, half_w);
            vertices.append(&mut pos.to_vec());

            let last_index0 = index_offset + ((i - 1) * 3) as u16;
            let cur_index0 = index_offset + (i * 3) as u16;
            indices.append(&mut vec![
                last_index0,
                last_index0 + 1,
                cur_index0,
                cur_index0,
                cur_index0 + 1,
                last_index0 + 1,
                cur_index0 + 1,
                last_index0 + 1,
                last_index0 + 2,
                cur_index0 + 1,
                last_index0 + 2,
                cur_index0 + 2,
            ]);

            last = &self.points[i];
        }
        (vertices, indices)
    }

    pub fn generate_points(&self) -> Vec<PosWeight> {
        let mut list: Vec<PosWeight> = vec![];
        for i in 0..self.points.len() {
            let p = self.points[i];
            list.push(PosWeight::new([p.x, p.y, 0.0], 0.0));
        }
        list
    }
}

// 计算点的连线距离
fn distance(points: &Vec<Position>) -> f32 {
    let mut d = 0.0;
    let mut last_p = points.first().unwrap();
    for i in 1..points.len() {
        d += points[i].distance(last_p);
        last_p = &points[i];
    }
    d
}

// 基于当前点及所在线段的斜率，计算线宽的两个端点
pub fn cal_pos(cur: &Position, original: f32, blend_radian: f32, half_w: f32) -> [PosBrush; 3] {
    let radian = -PI_2 + blend_radian;
    let other = -std::f32::consts::PI + radian;
    let mut p0 = PosBrush::new(
        [cur.x + half_w * radian.cos(), cur.y + half_w * radian.sin(), 0.0],
        [0.0, 0.5],
        [half_w, 0.0, 0.0],
    );
    let mut p1 = PosBrush::new(
        [cur.x + half_w * other.cos(), cur.y + half_w * other.sin(), 0.0],
        [0.0, 0.5],
        [half_w, 0.0, 0.0],
    );
    [p0, PosBrush::new([cur.x, cur.y, 0.0], [0.0, 0.5], [0.0, 0.0, 0.0]), p1]
}
