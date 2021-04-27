use idroid::{math::Position, math::TouchPoint, SurfaceView};
use sdf_text_view::SDFTextView;
use uni_view::AppView;

fn main() {
    use winit::event::{
        ElementState, Event, KeyboardInput, MouseScrollDelta, VirtualKeyCode, WindowEvent,
    };
    use winit::{event_loop::ControlFlow, event_loop::EventLoop, window::Window};

    let events_loop = EventLoop::new();
    let window = Window::new(&events_loop).unwrap();
    // window.set_max_dimensions(Some((400_u32, 700_u32).into()));
    // window.set_inner_size((900_u32, 600_u32).into());
    // window.set_max_inner_size(Some((2800_u32, 1850_u32).into()));
    window.set_title("SDF Text View");

    let v = AppView::new(window);

    let mut surface_view = SDFTextView::new(v);
    surface_view.bundle_image("math3.png".to_string(), false);

    events_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            Event::MainEventsCleared => surface_view.app_view.view.request_redraw(),
            Event::WindowEvent { event: WindowEvent::Resized(_size), .. } => {
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
                    *control_flow = winit::event_loop::ControlFlow::Exit;
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
                    let mut point = TouchPoint::new(
                        Position::new(position.x as f32, position.y as f32),
                        0.0,
                        0.0,
                        0.0,
                    );

                    surface_view.touch_moved(point);
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                surface_view.enter_frame();
            }
            _ => (),
        }
    });

    // let triangle = idroid::Triangle::new()
}
