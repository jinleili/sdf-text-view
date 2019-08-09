use crate::framework::CanvasView;
use crate::math::{Position, Rect, ViewSize};
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosWeight};

use crate::geometry::Line;

use nalgebra_glm as glm;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct LineUniform {
    half_width: f32,
    lookup_table: [f32; 32],
}

pub struct LineAntialiasing {
    view_size: ViewSize,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    mvp_buf: wgpu::Buffer,
    buf: wgpu::Buffer,
    uniform: LineUniform,
    pipeline: wgpu::RenderPipeline,
    depth_texture_view: wgpu::TextureView,
    is_animating: bool,
    is_touch_moving: bool,
    touch_start: Position,
    last_touch: Position,
    rotate: f32,
}

fn generate_mv_matrix(translate_z: f32) -> glm::TMat4<f32> {
    let mut vm_matrix = glm::TMat4::identity();
    vm_matrix = glm::translate(&vm_matrix, &glm::vec3(0.0, 0.0, translate_z));
    vm_matrix
}

impl CanvasView for LineAntialiasing {
    fn init(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self {
        use std::mem;
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
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

        let p_matrix: glm::TMat4<f32> =
            crate::matrix_helper::ortho_pixel(sc_desc.width as f32, sc_desc.height as f32);
        let mvp_uniform = MVPUniform {
            mvp_matrix: (p_matrix * generate_mv_matrix(-400.0)).into(),
        };
        let mvp_buf = crate::utils::create_uniform_buffer(device, mvp_uniform);

        let view_size = ViewSize {
            width: sc_desc.width as f32,
            height: sc_desc.height as f32,
        };

        let uniform = LineUniform {
            lookup_table: crate::utils::gaussian::lookup_table(),
            half_width: 10.0,
        };
        let buf = crate::utils::create_uniform_buffer(device, uniform);

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
                        buffer: &buf,
                        range: 0 .. (std::mem::size_of::<LineUniform>() as wgpu::BufferAddress),
                    },
                },
            ],
        });

        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<PosWeight>();
        let (vertex_data, index_data) = Line::new(
            Position { x: -300.0, y: 0.0 },
            Position { x: 300.0, y: 5.0 },
            20,
        )
        .generate_vertices();
        let vertex_buf = device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertex_data);

        let index_buf = device
            .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&index_data);

        // Create the render pipeline
        let shader = crate::shader::Shader::new("brush/line", device);
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
                color_blend: crate::utils::color_blend(),
                alpha_blend: crate::utils::alpha_blend(),
                write_mask: wgpu::ColorWrite::ALL,
            }],
            // ??????
            // depth_stencil_state: None,
            depth_stencil_state: Some(crate::depth_stencil::create_state_descriptor()),
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: vertex_size as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &PosWeight::attri_descriptor(0),
            }],
            sample_count: 1,
        });

        // Done
        let init_command_buf = encoder.finish();
        device.get_queue().submit(&[init_command_buf]);

        let depth_texture_view = crate::depth_stencil::create_depth_texture_view(sc_desc, device);

        LineAntialiasing {
            view_size,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            mvp_buf,
            buf,
            uniform,
            pipeline,
            depth_texture_view,
            is_animating: false,
            is_touch_moving: false,
            touch_start: Position::zero(),
            last_touch: Position::zero(),
            rotate: 0.0,
        }
    }

    fn update(&mut self, _event: wgpu::winit::WindowEvent) {
        //empty
    }

    fn touch_moved(&mut self, position: Position, device: &mut wgpu::Device) {}

    fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) {
        self.view_size = ViewSize {
            width: sc_desc.width as f32,
            height: sc_desc.height as f32,
        };
        // crate::utils::update_uniform(device, generate_uniforms(sc_desc), &self.uniform_buf);
        self.depth_texture_view = crate::depth_stencil::create_depth_texture_view(sc_desc, device);
    }

    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) {
        self.rotate += 0.5;
        let radian: glm::TVec1<f32> = glm::radians(&glm::vec1(self.rotate));

        let p_matrix: glm::TMat4<f32> =
            crate::matrix_helper::ortho_pixel(self.view_size.width, self.view_size.height);
        let mut vm_matrix: glm::TMat4<f32> = glm::TMat4::identity();
        vm_matrix = glm::rotate(&vm_matrix, radian[0], &glm::vec3(0.0, 0.0, 1.0));

        let mvp_uniform = MVPUniform {
            mvp_matrix: (p_matrix * vm_matrix).into(),
        };
        crate::utils::update_uniform(device, mvp_uniform, &self.mvp_buf);

        self.fresh_frame_data(device);

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
    }
}

impl LineAntialiasing {
    fn touch_end(&mut self) {}

    // 动画帧插值
    fn interpolate_frame_data(&mut self, target_pos: Position, is_open: bool) {}

    fn fresh_frame_data(&mut self, device: &mut wgpu::Device) {}

    fn step_frame_data(&mut self, position: Position, device: &mut wgpu::Device) {}
}
