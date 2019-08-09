#![allow(dead_code)]

use gleam::gl::*;

pub struct TiledSprites {
    h_num: u32,
    v_num: u32,
    pub vertices: Vec<GLfloat>,
    pub tex_coords: Vec<GLfloat>,
}

impl TiledSprites {
    pub fn new(width: GLfloat, height: GLfloat, size: GLfloat) -> Self {
        // 不用 ceil, 避免右上两边的 sprite 采样图像不准确
        let h_num = (width / size).floor() as u32;
        let v_num = (height / size).floor() as u32;
        let half_size = size / 2.0;

        let mut vertices: Vec<GLfloat> = Vec::new();
        let mut coords: Vec<GLfloat> = Vec::new();

        // 下边的写法等同于 for (let h=0; h<(h_segments + 1); h++) {}
        for h in 0..h_num {
            let x: GLfloat = half_size + size * (h as GLfloat);
            for v in 0..v_num {
                let y: GLfloat = half_size + size * (v as GLfloat);
                vertices.push(x);
                vertices.push(y);
                coords.push(x / width);
                coords.push(y / height);
            }
        }

        TiledSprites {
            h_num: h_num,
            v_num: v_num,
            vertices: vertices,
            tex_coords: coords,
        }
    }

    fn get_vertices_speed(&self) -> Vec<GLfloat> {
        let mut speeds: Vec<GLfloat> = Vec::new();
        for h in 0..self.h_num {
            for v in 0..self.v_num {}
        }
        speeds
    }
}
