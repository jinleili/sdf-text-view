extern crate idroid;
use idroid::{math::Position, SurfaceView};

extern crate uni_view;
use uni_view::AppView;

extern crate lazy_static;
extern crate objc;

extern crate nalgebra_glm;
use nalgebra_glm as glm;

use std::time::{Duration, Instant};

fn main() {
    use wgpu::winit::{
        ElementState, Event, EventsLoop, KeyboardInput, MouseScrollDelta, VirtualKeyCode, Window,
        WindowEvent,
    };

    env_logger::init();

    let mut events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();
    // window.set_max_dimensions(Some((400_u32, 700_u32).into()));
    window.set_max_dimensions(Some((1800_u32, 350_u32).into()));
    window.set_title("title");

    // let screen_scale: fn() -> f32 = screen_scale;
    let v = AppView::new(window);

    let mut surface_view = idroid::SDFTextView::new(v);
    // let mut surface_view = idroid::filters::GrayFilter::new(v);

    let mut running = true;
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();
    let fixed_time_stamp = Duration::new(0, 16666667);

    // test_projection();
    while running {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event: WindowEvent::Resized(_size), .. } => {
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
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(_x, y) => {
                        println!("{:?}, {}", _x, y);
                    }
                    _ => (),
                },
                WindowEvent::Touch(touch) => {
                    println!("{:?}", touch);
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
