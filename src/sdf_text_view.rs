use crate::geometry::plane::Plane;
use crate::texture;
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosTex};
use crate::SurfaceView;

use uni_view::{AppView, GPUContext};

// use nalgebra_glm as glm;

pub struct SDFTextView {
    app_view: AppView,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    mvp_buf: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

impl SDFTextView {
    pub fn new(app_view: AppView) -> Self {
        let mut app_view = app_view;

        let mut encoder =
            app_view.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        // Create the texture
        let (texture_view, _texture_extent, sampler) = texture::from_file_and_usage_write(
            "math.png",
            &mut app_view.device,
            &mut encoder,
            true,
        );

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
        // let mvp = crate::matrix_helper::ortho_pixel_mvp(app_view.sc_desc.width as f32, app_view.sc_desc.height as f32);
        let mvp = crate::matrix_helper::default_mvp(&app_view.sc_desc);
        let mvp_buf = crate::utils::create_uniform_buffer(&mut app_view.device, mvp);
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
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::Binding { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });

        // Create the vertex and index buffers
        let vertex_size = std::mem::size_of::<PosTex>();
        let (vertex_data, index_data) = Plane::new(1, 1).generate_vertices();
        // let index_data = [3, 1, 1, 1, 1, 1];
        println!("vertex_data: {:?}", &vertex_data );

        let vertex_buf = app_view
            .device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertex_data);

        let index_buf = app_view
            .device
            .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&index_data);
        // Create the render pipeline
        let shader = crate::shader::Shader::new("sdf/text", &mut app_view.device);
        let pipeline_layout =
            app_view.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            });
        let pipeline = app_view.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: shader.vertex_stage(),
            fragment_stage: shader.fragment_stage(),
            rasterization_state: wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            },
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: app_view.sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            // ??????
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint32,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: vertex_size as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &PosTex::attri_descriptor(0),
            }],
            sample_count: 1,
        });
        app_view.device.get_queue().submit(&[encoder.finish()]);

        SDFTextView {
            app_view,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            pipeline,
            mvp_buf,
        }
    }
}

impl SurfaceView for SDFTextView {
    fn update(&mut self, _event: wgpu::winit::WindowEvent) {
        //empty
    }

    fn touch_moved(&mut self, _position: crate::math::Position) {}

    fn resize(&mut self) {
        self.app_view.update_swap_chain();
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
                    depth_stencil_attachment: None,
                });
                rpass.set_pipeline(&self.pipeline);
                rpass.set_bind_group(0, &self.bind_group, &[]);
                rpass.set_index_buffer(&self.index_buf, 0);
                rpass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
                rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
                // rpass.draw_indexed(0..3, 0, 0..1);

            }

            self.app_view.device.get_queue().submit(&[encoder.finish()]);
        }
    }
}
