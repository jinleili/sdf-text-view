use std::f32;
use std::fs::File;
use std::io::Write;
use std::vec::Vec;

// use std::f32::{INFINITY, NEG_INFINITY};

use image::GrayImage;

static INF: f32 = f32::MAX;

pub struct SDF {
    output_image_path: String,
    cutoff: f32,
    radius: f32,
    img: GrayImage,
    img_size: (u32, u32),
    pixel_count: usize,
    long_edge_pixel: usize,
    f: Vec<f32>,
}

impl SDF {
    pub fn new(input_image_path: &str, output_image_path: &str) -> Self {
        let mut img: GrayImage =
            image::open(input_image_path).ok().expect("failed to load image").to_luma();
        let img_size = img.dimensions();
        let pixel_count = (img_size.0 * img_size.1) as usize;
        let long_edge_pixel =
            if img_size.0 > img_size.1 { img_size.0 as usize } else { img_size.1 as usize };

        let cutoff = 0.25;
        let radius = 8.0;

        let f: Vec<f32> = vec![0.0; long_edge_pixel];

        SDF {
            output_image_path: output_image_path.to_string(),
            cutoff,
            radius,
            img,
            img_size,
            long_edge_pixel,
            pixel_count,
            f,
        }
    }

    pub fn generate(&mut self) {
        // temporary arrays for the distance transform
        let mut grid_outer: Vec<f32> = vec![0.0; self.pixel_count];
        let mut grid_inner: Vec<f32> = vec![0.0; self.pixel_count];
        let mut luma_channel: Vec<u8> = vec![0; self.pixel_count];
        for ix in 0..self.img_size.0 {
            for iy in 0..self.img_size.1 {
                let luma = self.img.get_pixel(ix, iy)[0] as f32 / 255.0;
                let index = self.img_index(ix as usize, iy as usize);
                if luma > 0.98 {
                    grid_inner[index] = INF;
                    grid_outer[index] = 0.0;
                } else if luma < 0.1 {
                    grid_inner[index] = 0.0;
                    grid_outer[index] = INF;
                } else {
                    grid_inner[index] = max(0.0, luma - 0.5).powf(2.0);
                    grid_outer[index] = max(0.0, 0.5 - luma).powf(2.0);
                }
            }
        }

        self.edt(&mut grid_outer);
        self.edt(&mut grid_inner);

        for i in 0..self.pixel_count {
            let d = grid_outer[i].sqrt() - grid_inner[i].sqrt();
            luma_channel[i] = (255.0 - 255.0 * (d / self.radius + self.cutoff)).round() as u8;
            // luma_channel[i] = (255.0 * (d + 1.0) / 2.0) as u8;
            // if d < 0.4 {
            //     print!("{:?}, {:?}, | ", d, luma_channel[i]);

            // }
        }

        let outf = File::create(&self.output_image_path).unwrap();
        let encoder = image::png::PNGEncoder::<std::fs::File>::new(outf);
        encoder
            .encode(&luma_channel, self.img_size.0, self.img_size.1, image::ColorType::Gray(8))
            .unwrap();
    }

    fn edt(&mut self, grid: &mut Vec<f32>) {
        let width = self.img_size.0 as usize;
        let height = self.img_size.1 as usize;
           // transform along columns
        for x in 0..width {
            for y in 0..height {
                self.f[y] = grid[self.img_index(x, y)];
            }
            let d = self.edt1d(grid, height);
            for y in 0..height {
                grid[self.img_index(x, y)] = d[y];
            }
        }
        // transform along rows
        for y in 0..height {
            for x in 0..width {
                self.f[x] = grid[self.img_index(x, y)];
            }
            let d = self.edt1d(grid, width);
            for x in 0..width {
                grid[self.img_index(x, y)] = d[x];
            }
        }
    }

    fn edt1d(&mut self, grid: &mut Vec<f32>, length: usize) -> Vec<f32> {
        let (mut k, mut s, mut r) = (0_usize, 0.0_f32, 0_usize);

        let mut v = vec![0_usize; length];
        let mut z = vec![0.0_f32; length + 1];
        let mut d = vec![0.0_f32; length];

        z[0] = -INF;
        z[1] = INF;

        for q in 1..length {
            loop {
                r = v[k] as usize;
                s = (self.f[q] + (q * q) as f32 - self.f[r] - (r * r) as f32)
                    / (2 * (q - r)) as f32;
                // 实际情况: k 不会小于 0
                if s <= z[k] {
                    k -= 1;
                } else {
                    break;
                }
            }
            k += 1;
            v[k] = q;
            z[k] = s;
            z[k + 1] = INF;
        }

        k = 0;
        for q in 0..length {
            while z[k + 1] < q as f32 {
                k += 1;
            }
            // q and r are usize, if q - r < 0, will cause panic: thread 'main' panicked at 'attempt to subtract with overflow'
            d[q] = self.f[v[k]] + (q as i32 - v[k] as i32).pow(2) as f32;
        }
        
        d
    }

    fn img_index(&self, x: usize, y: usize) -> usize {
        y * self.img_size.0 as usize  + x
    }
}

fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
