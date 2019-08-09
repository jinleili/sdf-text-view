// 希尔伯特曲线
// https://mp.weixin.qq.com/s?__biz=MzA5OTgyMDk3Mg==&mid=2651226513&idx=1&sn=d0a7ca19c2472fdf84066d50806d97af&chksm=8b0e5552bc79dc44b9b1e661c8a65f475cc12d814decd0c920dee8eee4f1122e19203f86258d&mpshare=1&scene=23&srcid=04087uUnkEMZacAS8Om9fkcY#rd
#![allow(dead_code)]

use gleam::gl::*;

// nalgebra 使用文档
// 变换：http://nalgebra.org/points_and_transformations/
extern crate nalgebra as na;
use self::na::{Point2, Similarity2, Vector2};

pub struct HilbertCurve {
    pub dimension: usize,
    vertices: Vec<GLfloat>,
    vertices_4: Vec<GLfloat>,
    // 是否为翻 4 倍的状态
    pub is_four_time_state: bool,
}

impl HilbertCurve {
    pub fn new(dimension: usize) -> Self {
        let mut instance = HilbertCurve {
            dimension: dimension,
            vertices: vec![],
            vertices_4: vec![],
            is_four_time_state: false,
        };
        instance.generate_mesh();
        instance
    }

    pub fn generate_mesh(&mut self) {
        if self.dimension == 0 {
            return;
        }

        let pi = 1.5707963;

        let mut base = vec![
            Point2::new(-0.5, -0.5),
            Point2::new(-0.5, 0.5),
            Point2::new(0.5, 0.5),
            Point2::new(0.5, -0.5),
        ];

        // 大于 1 维的希尔博特曲线，点的变换添加由左下角 -> 左上角 -> 右上角 -> 右下角
        for _ in 1..self.dimension {
            let mut new_points: Vec<Point2<f32>> = vec![];
            for i in 0..=3 {
                // 变换矩阵每个方位不一样
                // 点的顺序在 左下角 及 右下角 时都需要反转
                // 点序的反转通过遍历 base 的顺序来实现
                let mut need_flip = false;
                let mat = match i {
                    0 => {
                        need_flip = true;
                        Similarity2::new(Vector2::new(-0.5, -0.5), -pi, 0.5)
                    }
                    1 => Similarity2::new(Vector2::new(-0.5, 0.5), 0., 0.5),
                    2 => Similarity2::new(Vector2::new(0.5, 0.5), 0., 0.5),
                    _ => {
                        need_flip = true;
                        Similarity2::new(Vector2::new(0.5, -0.5), pi, 0.5)
                    }
                };
                // 这种遍历无法倒序进行
                for k in 0..base.len() {
                    let mut p: Point2<f32>;
                    if need_flip {
                        p = base[(base.len() - 1) - k].clone();
                    } else {
                        p = base[k].clone();
                    }
                    p = mat * p;
                    new_points.push(p);
                }
            }
            base = new_points;
        }

        // 输出最终顶点
        let mut out_array: Vec<f32> = vec![];
        for i in 0..base.len() {
            let p = base[i];
            out_array.push(p[0]);
            out_array.push(p[1]);
        }
        self.vertices = out_array;
    }

    pub fn four_times_vertices(&mut self) {
        self.is_four_time_state = true;
        if self.vertices_4.len() > 0 {
            return;
        }
        let mut out_array: Vec<f32> = vec![];
        for i in 0..self.vertices.len() / 2 {
            let px = self.vertices[i * 2];
            let py = self.vertices[i * 2 + 1];
            // let step_x = (self.vertices[(i + 1) * 2] - px) / 3.;
            out_array.push(px);
            out_array.push(py);
            out_array.push(px);
            out_array.push(py);
            out_array.push(px);
            out_array.push(py);
            out_array.push(px);
            out_array.push(py);
        }
        self.vertices_4 = out_array;
    }

    pub fn get_vertices(&self) -> &Vec<GLfloat> {
        if self.is_four_time_state {
            &self.vertices_4
        } else {
            &self.vertices
        }
    }
}
