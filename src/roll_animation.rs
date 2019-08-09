use crate::texture;
use crate::vertex::{Pos, PosTex};

use crate::geometry::plane::Plane;
use crate::SurfaceView;

use uni_view::{AppView, GPUContext};

use nalgebra_glm as glm;

#[repr(C)]
#[derive(Clone, Copy)]
struct RollUniforms {
    mvp_matrix: [[f32; 4]; 4],
    roll_to: f32,
    start_radius: f32,
}

pub struct RollAnimation {
    app_view: AppView,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    depth_texture_view: wgpu::TextureView,
}

fn generate_uniforms(sc_desc: &wgpu::SwapChainDescriptor) -> RollUniforms {
    //
    let radian: glm::TVec1<f32> = glm::radians(&glm::vec1(75.0));
    let p_matrix: glm::TMat4<f32> =
        glm::perspective_fov(radian[0], sc_desc.width as f32, sc_desc.height as f32, 0.01, 100.0);
    //        let mut  p_matrix: glm::TMat4<f32> = glm::ortho(-1.0, 1.0, -1.0, 1.0, -100.0, 100.0);
    let mut vm_matrix = glm::TMat4::identity();
    vm_matrix = glm::translate(&vm_matrix, &glm::vec3(0.0, 0.0, -6.0));
    vm_matrix = glm::scale(&vm_matrix, &glm::vec3(1.0, 2.0, 2.0));
    vm_matrix = glm::rotate(&vm_matrix, radian[0], &glm::vec3(0.0, 1.0, 0.0));
    RollUniforms { mvp_matrix: (p_matrix * vm_matrix).into(), roll_to: 1.6, start_radius: 0.13 }
}

impl RollAnimation {
    pub fn new(app_view: AppView) -> Self {
        let mut app_view = app_view;
        use std::mem;
        let mut encoder =
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
                        ty: wgpu::BindingType::SampledTexture,
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 2,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler,
                    },
                ],
            });
        let pipeline_layout =
            app_view.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            });
        println!("--Create pipeline layout");

        // Create the texture
        let (texture_view, _texture_extent, sampler) =
            texture::from_file("512*1024.png", &mut app_view.device, &mut encoder);
        println!("--Create the texture");

        // Create other resources
        let uniform_buf = crate::utils::create_uniform_buffer(
            &mut app_view.device,
            generate_uniforms(&app_view.sc_desc),
        );

        println!("size: {}", std::mem::size_of::<RollUniforms>());
        // Create bind group
        let bind_group = app_view.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0..(std::mem::size_of::<RollUniforms>() as wgpu::BufferAddress),
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::Binding { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });
        println!("--create_bind_group");

        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<PosTex>();
        let (vertex_data, index_data) = Plane::new(2, 100).generate_vertices();
        let vertex_buf = app_view
            .device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertex_data);

        let index_buf = app_view
            .device
            .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&index_data);

        // Create the render pipeline
        let shader = crate::shader::Shader::new("roll", &mut app_view.device);
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
            index_format: wgpu::IndexFormat::Uint16,
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
        RollAnimation {
            app_view,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            uniform_buf,
            pipeline,
            depth_texture_view,
        }
    }
}

impl SurfaceView for RollAnimation {
    fn update(&mut self, _event: wgpu::winit::WindowEvent) {
        //empty
    }

    fn touch_moved(&mut self, _position: crate::math::Position) {}

    fn resize(&mut self) {
        self.app_view.update_swap_chain();
        crate::utils::update_uniform(
            &mut self.app_view.device,
            generate_uniforms(&self.app_view.sc_desc),
            &self.uniform_buf,
        );
        self.depth_texture_view = crate::depth_stencil::create_depth_texture_view(
            &self.app_view.sc_desc,
            &mut self.app_view.device,
        );
    }

    fn enter_frame(&mut self) {
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
    }
}
