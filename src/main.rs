extern crate lazy_static;

// pub mod framework;
// pub mod geometry;
// pub mod node;
// pub mod texture;
// pub mod utils;
// pub mod vertex;

// mod brush;
// mod cube_example;
// mod depth_stencil;
// mod filters;
// mod fractal;
// mod math;
// mod matrix_helper;
// mod page_turning;
// mod procedure_texture;
// mod roll_animation;

// #[cfg(not(target_os = "ios"))]
// mod shader;

// use crate::cube_example::CubeExample;
// use crate::filters::BlurFilter;
// use crate::page_turning::PageTurning;
// use crate::roll_animation::RollAnimation;
// use brush::{BrushView, LineAntialiasing};
// use fractal::Mandelbrot;
// use procedure_texture::Brick;

fn main() {
    // framework::run::<CubeExample>("cube");
    // framework::run::<RollAnimation>("roll");
    // framework::run::<BlurFilter>("iOS 模糊效果");
    // framework::run::<PageTurning>("仿真翻页动画");
    // framework::run::<Brick>("无限滚动的砖墙");
    // framework::run::<Mandelbrot>("曼德布罗集");
    // framework::run::<LineAntialiasing>("线段抗锯齿");
    // framework::run:1:<BrushView>("笔刷");
}

#[test]
fn test_line_segment() {
    use idroid::math::LineSegment;

    let l0 = LineSegment::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let l1 = LineSegment::new([0.0, 0.0, 0.0], [0.0, 2.0, 0.0]);
    let l2 = LineSegment::new([0.0, 0.0, 0.0], [2.0, 2.0, 0.0]);
    let l3 = LineSegment::new([0.0, 0.0, 0.0], [1.0, 1.0, 0.0]);
    let l4 = LineSegment::new([0.0, 0.0, 0.0], [1.0, 2.0, 0.0]);
    let l5 = LineSegment::new([0.0, 0.0, 0.0], [1.0, 3.0, 0.0]);
    let l6 = LineSegment::new([1.0, 0.0, 0.0], [1.0, 2.0, 0.0]);

    let r0 = LineSegment::new([0.0, 1.0, 0.0], [0.0, 3.0, 0.0]);
    let r1 = LineSegment::new([1.0, 1.0, 0.0], [3.0, 3.0, 0.0]);
    let r2 = LineSegment::new([2.0, 2.0, 0.0], [3.0, 3.0, 0.0]);
    let r3 = LineSegment::new([1.0, 0.0, 0.0], [2.0, 1.0, 0.0]);
    let r4 = LineSegment::new([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let r5 = LineSegment::new([0.0, 3.0, 0.0], [2.0, 3.0, 0.0]);
    let r6 = LineSegment::new([1.0, 1.0, 0.0], [2.0, 1.0, 0.0]);

    assert_eq!(false, l0.is_intersect_with(&r0));
    assert_eq!(false, l1.is_intersect_with(&r0));

    assert_eq!(false, l2.is_intersect_with(&r1));
    assert_eq!(false, l3.is_intersect_with(&r2));
    assert_eq!(false, l3.is_intersect_with(&r3));

    assert_eq!(true, l2.is_intersect_with(&r4));
    assert_eq!(true, l4.is_intersect_with(&r4));
    assert_eq!(true, l5.is_intersect_with(&r4));

    assert_eq!(false, l6.is_intersect_with(&r5));
    assert_eq!(false, l1.is_intersect_with(&r6));

    if let Some(v) = l2.intersect_with(&r4) {
        println!("{:?}", v);
    }
    if let Some(v) = l4.intersect_with(&r4) {
        println!("{:?}", v);
    }
    if let Some(v) = l5.intersect_with(&r4) {
        println!("{:?}", v);
    }
}
