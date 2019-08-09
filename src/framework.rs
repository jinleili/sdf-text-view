use crate::math::Position;
use log::info;
use std::time::{Duration, Instant};
// use lazy_static::*;

lazy_static! {
    static ref instance: wgpu::Instance = wgpu::Instance::new();
}
// 实现 static mut : https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton/27826181
// use std::sync::Mutex;
// lazy_static! {
//     static ref device: Mutex<wgpu::Device> = Mutex::new(create_device());
// }
// fn create_device() -> wgpu::Device {
//     let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
//         power_preference: wgpu::PowerPreference::LowPower,
//     });
//     adapter.create_device(&wgpu::DeviceDescriptor {
//         extensions: wgpu::Extensions {
//             anisotropic_filtering: false,
//         },
//     })
// }

#[allow(dead_code)]
pub fn cast_slice<T>(data: &[T]) -> &[u8] {
    use std::mem::size_of;
    use std::slice::from_raw_parts;

    unsafe { from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

pub trait CanvasView {
    fn init(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self;
    fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device);
    fn update(&mut self, event: wgpu::winit::WindowEvent);
    fn touch_moved(&mut self, position: crate::math::Position, device: &mut wgpu::Device);
    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device);
}

pub fn run<E: CanvasView>(title: &str) {
    use wgpu::winit::{
        ElementState, Event, EventsLoop, KeyboardInput, VirtualKeyCode, Window, WindowEvent,
    };

    env_logger::init();
    // let instance = wgpu::Instance::new();
    let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
        power_preference: wgpu::PowerPreference::LowPower,
    });
    let mut device = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    let mut events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();
    window.set_max_dimensions(Some((400_u32, 700_u32).into()));

    window.set_title(title);

    let scale_factor = window.get_hidpi_factor();
    let size = window.get_inner_size().unwrap().to_physical(scale_factor);

    let surface = instance.create_surface(&window);

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width as u32,
        height: size.height as u32,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);
    let mut canvas = E::init(&sc_desc, &mut device);

    let mut running = true;
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();
    let fixed_time_stamp = Duration::new(0, 16666667);

    while running {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let physical = size.to_physical(window.get_hidpi_factor());
                println!("Resizing to {:?}", physical);
                sc_desc.width = physical.width as u32;
                sc_desc.height = physical.height as u32;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
                canvas.resize(&sc_desc, &mut device);
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
                    let scale = scale_factor as f32;
                    canvas.touch_moved(
                        Position::new(position.x as f32 * scale, position.y as f32 * scale),
                        &mut device,
                    );
                }
                _ => {
                    canvas.update(event);
                }
            },
            _ => (),
        });

        let now = Instant::now();
        accumulator += now - previous_clock;

        if accumulator >= fixed_time_stamp {
            previous_clock = now;
            let frame = swap_chain.get_next_texture();
            canvas.render(&frame, &mut device);
        } else {
            std::thread::sleep(fixed_time_stamp - accumulator);
        }

        running &= !cfg!(feature = "metal-auto-capture");
    }
}
