use crate::math::Position;
use crate::AppViewWrapper;
use std::time::{Duration, Instant};

extern crate lazy_static;
use lazy_static::*;

use uni_view::{AppView, GPUContext};

lazy_static! {
    static ref instance: wgpu::Instance = wgpu::Instance::new();
}

pub trait SurfaceView {
    fn resize(&mut self);
    fn update(&mut self, event: wgpu::winit::WindowEvent);
    fn touch_moved(&mut self, position: Position);

    fn enter_frame(&mut self);
}

use crate::geometry::plane::Plane;
use crate::math::ViewSize;
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosTex};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct OffsetUniform {
    brick_offset: f32,
    angle: f32,
    last_angle: f32,
}

pub struct Triangle {
    app_view: AppView,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    offset_buf: wgpu::Buffer,
    offset_uniform: OffsetUniform,
    pipeline: wgpu::RenderPipeline,
    depth_texture_view: wgpu::TextureView,
    frame_gap: u32,
}

impl Triangle {
    pub fn new(app_view: AppView) -> Self {
        let mut app_view = app_view;
        // let mut device = app_view.get_device(&instance);
        let size = app_view.get_view_size();
        // let sc_desc = wgpu::SwapChainDescriptor {
        //     usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        //     format: wgpu::TextureFormat::Bgra8Unorm,
        //     width: size.width,
        //     height: size.height,
        // };

        // let surface = app_view.get_surface(&instance);
        // let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        println!("swap_chain");
        use std::mem;
        let encoder =
            app_view.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        // Create pipeline layout
        let bind_group_layout =
            app_view.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        let pipeline_layout =
            app_view.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            });

        let mvp = crate::matrix_helper::ortho_default_mvp();
        let mvp_buf = crate::utils::create_uniform_buffer(
            &mut app_view.device,
            MVPUniform { mvp_matrix: mvp },
        );

        let offset_uniform = OffsetUniform { brick_offset: 0.0, angle: 0.0, last_angle: 0.0 };
        let offset_buf = crate::utils::create_uniform_buffer(&mut app_view.device, offset_uniform);

        // Create bind group
        let bind_group = app_view.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &mvp_buf,
                        range: 0..(std::mem::size_of::<MVPUniform>() as wgpu::BufferAddress),
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &offset_buf,
                        range: 0..(std::mem::size_of::<OffsetUniform>() as wgpu::BufferAddress),
                    },
                },
            ],
        });

        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<PosTex>();
        let (vertex_data, index_data) = Plane::new(1, 1).generate_vertices();
        let vertex_buf = app_view
            .device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertex_data);
        let index_buf = app_view
            .device
            .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&index_data);

        println!("shader 0");

        // Create the render pipeline
        let shader = crate::shader::Shader::new("procedual/brick", &mut app_view.device);
        println!("shader 1");

        let pipeline = app_view.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                format: app_view.sc_desc.format,
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
        app_view.device.get_queue().submit(&[init_command_buf]);

        let depth_texture_view = crate::depth_stencil::create_depth_texture_view(
            &app_view.sc_desc,
            &mut app_view.device,
        );

        Triangle {
            app_view,
            // device,
            // surface,
            // sc_desc,
            // swap_chain,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            offset_buf,
            offset_uniform,
            pipeline,
            depth_texture_view,
            frame_gap: 0,
        }
    }
}

impl SurfaceView for Triangle {
    fn resize(&mut self) {
        self.app_view.update_swap_chain();

        self.depth_texture_view = crate::depth_stencil::create_depth_texture_view(
            &self.app_view.sc_desc,
            &mut self.app_view.device,
        );
    }

    fn update(&mut self, event: wgpu::winit::WindowEvent) {}
    fn touch_moved(&mut self, position: Position) {}

    fn enter_frame(&mut self) {
        println!("--- enter frame ---");
        let mut encoder = self
            .app_view
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let frame = self.app_view.swap_chain.get_next_texture();

            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: crate::utils::clear_color(),
                    }],
                    depth_stencil_attachment: Some(
                        crate::depth_stencil::create_attachment_descriptor(
                            &self.depth_texture_view,
                        ),
                    ),
                });
                rpass.set_pipeline(&self.pipeline);
                rpass.set_bind_group(0, &self.bind_group, &[]);
                rpass.set_index_buffer(&self.index_buf, 0);
                rpass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
                rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
            }
            self.app_view.device.get_queue().submit(&[encoder.finish()]);
        }

        self.step_frame_data();
    }
}

impl Triangle {
    fn step_frame_data(&mut self) {
        if self.offset_uniform.last_angle == self.offset_uniform.angle && self.frame_gap >= 30 {
            self.offset_uniform.brick_offset += 0.05;
            crate::utils::update_uniform(
                &mut self.app_view.device,
                self.offset_uniform,
                &self.offset_buf,
            );
        } else {
            self.offset_uniform.last_angle = self.offset_uniform.angle;
            self.frame_gap += 1;
        }
        println!("{}", self.offset_uniform.brick_offset);
    }
}
