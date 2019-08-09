extern crate idroid;
use idroid::{math::Position, SurfaceView};

extern crate uni_view;
use uni_view::{AppView, ViewSize};

extern crate lazy_static;
extern crate objc;

extern crate nalgebra_glm;
use nalgebra_glm as glm;

use log::info;
use std::time::{Duration, Instant};

fn main() {
    use wgpu::winit::{
        ElementState, Event, EventsLoop, KeyboardInput, VirtualKeyCode, Window, WindowEvent,
    };

    env_logger::init();

    let mut events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();
    window.set_max_dimensions(Some((400_u32, 700_u32).into()));
    window.set_title("title");

    // let screen_scale: fn() -> f32 = screen_scale;
    let v = AppView::new(window);
    // let mut surfaceView = idroid::Triangle::new(idroid::AppViewWrapper(v));
    // let mut surfaceView = idroid::Triangle::new(v);
    // let mut surface_view = idroid::procedure_texture::Brick::new(v);
    // let mut surface_view = idroid::filters::BlurFilter::new(v);
    // let mut surface_view = idroid::PageTurning::new(v);
    // let mut surface_view = idroid::RollAnimation::new(v);
    // let mut surface_view = idroid::BrushView::new(v);
    // let mut surface_view = idroid::filters::GrayFilter::new(v);
    let mut surface_view = idroid::fluid2::PoiseuilleFlow::new(v);

    let mut running = true;
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();
    let fixed_time_stamp = Duration::new(0, 16666667);

    // test_projection();
    while running {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                // let physical = size.to_physical(window.get_hidpi_factor());
                // println!("Resizing to {:?}", physical);
                surface_view.resize();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    running = false;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    surface_view.touch_moved(Position::new(position.x as f32, position.y as f32));
                }
                _ => {}
            },
            _ => (),
        });

        let now = Instant::now();
        accumulator += now - previous_clock;

        if accumulator >= fixed_time_stamp {
            previous_clock = now;
            accumulator = Duration::new(0, 0);
            surface_view.enter_frame();
        } else {
            std::thread::sleep(fixed_time_stamp - accumulator);
        }

        running &= !cfg!(feature = "metal-auto-capture");
    }
    // let triangle = idroid::Triangle::new()
}

fn test_projection() {
    let mut vm_matrix = glm::TMat4::identity();
    // vm_matrix = glm::translate(&vm_matrix, &glm::vec3(0.0, 0.0, -10.0));
    let p_matrix: glm::TMat4<f32> = idroid::matrix_helper::ortho_pixel(400.0, 400.0);
    let v = glm::TVec4::new(100.0, -200.0, 0.0, 1.0);

    let arr: [[f32; 4]; 4] = (p_matrix * vm_matrix).into();
    println!("{:?}", arr);

    println!("{:?}", p_matrix * v);
}
