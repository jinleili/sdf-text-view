extern crate idroid;
use idroid::{math::Position, SurfaceView};

extern crate uni_view;
use uni_view::AppView;

extern crate lazy_static;
extern crate objc;

fn main() {
    use winit::event::{
        ElementState, Event, KeyboardInput, MouseScrollDelta, VirtualKeyCode, WindowEvent,
    };
    use winit::{event_loop::EventLoop, window::Window};

    env_logger::init();

    let events_loop = EventLoop::new();
    let window = Window::new(&events_loop).unwrap();
    // window.set_max_dimensions(Some((400_u32, 700_u32).into()));
    window.set_max_inner_size(Some((1800_u32, 1850_u32).into()));
    window.set_title("SDF Text View");

    let v = AppView::new(window);

    let mut surface_view = idroid::SDFTextView::new(v);
    // winit 0.20.0-alpha3 不会主动触发 WindowEvent::Resized 事件了
   

    events_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            winit::event_loop::ControlFlow::Exit
        } else {
            winit::event_loop::ControlFlow::Poll
        };
        match event {
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
                    surface_view.touch_moved(Position::new(position.x as f32, position.y as f32));
                }
                _ => {}
            },
            Event::EventsCleared => {
                surface_view.enter_frame();
            }
            _ => (),
        }
    });

    // let triangle = idroid::Triangle::new()
}
