use crate::framework::CanvasView;
use crate::geometry::plane::Plane;
use crate::math::{Position, ViewSize};
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosTex};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct JuliaUniform {
    screen: [f32; 2],
    const_c: [f32; 2],
    hue: f32,
    brightness: f32,
}

pub struct Mandelbrot {
    view_size: ViewSize,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    uniform: JuliaUniform,
    pipeline: wgpu::RenderPipeline,
    depth_texture_view: wgpu::TextureView,
    frame_gap: u32,
}

impl CanvasView for Mandelbrot {
    fn init(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self {
        use std::mem;
        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        // Create pipeline layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer,
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let mvp = crate::matrix_helper::ortho_default_mvp();
        let mvp_buf = crate::utils::create_uniform_buffer(device, MVPUniform { mvp_matrix: mvp });

        let view_size = ViewSize {
            width: sc_desc.width as f32,
            height: sc_desc.height as f32,
        };

        let uniform = JuliaUniform {
            screen: view_size.into(),
            const_c: [0.0, 0.0],
            hue: 0.9,
            brightness: 0.65,
        };
        let uniform_buf = crate::utils::create_uniform_buffer(device, uniform);

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &mvp_buf,
                        range: 0 .. (std::mem::size_of::<MVPUniform>() as wgpu::BufferAddress),
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0 .. (std::mem::size_of::<JuliaUniform>() as wgpu::BufferAddress),
                    },
                },
            ],
        });

        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<PosTex>();
        let (vertex_data, index_data) = Plane::new(1, 1).generate_vertices();
        let vertex_buf = device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertex_data);
        let index_buf = device
            .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&index_data);

        // Create the render pipeline
        let shader = crate::shader::Shader::new("fractal/mandelbrot", device);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: shader.vertex_stage(),
            fragment_stage: shader.fragment_stage(),
            rasterization_state: wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Cw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            },
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            // primitive_topology: wgpu::PrimitiveTopology::LineList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            // ??????
            // depth_stencil_state: None,
            depth_stencil_state: Some(crate::depth_stencil::create_state_descriptor()),
            index_format: wgpu::IndexFormat::Uint32,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: vertex_size as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &PosTex::attri_descriptor(0),
            }],
            sample_count: 1,
        });

        // Done
        let init_command_buf = encoder.finish();
        device.get_queue().submit(&[init_command_buf]);

        let depth_texture_view = crate::depth_stencil::create_depth_texture_view(sc_desc, device);

        Mandelbrot {
            view_size,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            uniform_buf,
            uniform,
            pipeline,
            depth_texture_view,
            frame_gap: 0,
        }
    }

    fn update(&mut self, _event: wgpu::winit::WindowEvent) {
        //empty
    }

    fn touch_moved(&mut self, position: Position, device: &mut wgpu::Device) {
        let p = position.ortho_in(self.view_size);
        let px = p.x / self.view_size.width * 4.0;
        let py = p.y / self.view_size.height * 4.0;
        let radian = py.atan2(px);
        let radius = (px.powf(2.0) + py.powf(2.0)).sqrt();
        self.uniform.const_c = [radian.cos() * radius, radian.sin() * radius];
        crate::utils::update_uniform(device, self.uniform, &self.uniform_buf);
    }

    fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) {
        self.view_size = ViewSize {
            width: sc_desc.width as f32,
            height: sc_desc.height as f32,
        };
        // crate::utils::update_uniform(device, generate_uniforms(sc_desc), &self.uniform_buf);
        self.depth_texture_view = crate::depth_stencil::create_depth_texture_view(sc_desc, device);
    }

    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: crate::utils::clear_color(),
                }],
                depth_stencil_attachment: Some(crate::depth_stencil::create_attachment_descriptor(
                    &self.depth_texture_view,
                )),
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_index_buffer(&self.index_buf, 0);
            rpass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
            rpass.draw_indexed(0 .. self.index_count as u32, 0, 0 .. 1);
        }
        device.get_queue().submit(&[encoder.finish()]);

        self.step_frame_data(device);
    }
}

impl Mandelbrot {
    fn step_frame_data(&mut self, device: &mut wgpu::Device) {}
}
