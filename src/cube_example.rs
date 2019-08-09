use crate::framework;
use crate::texture;
use crate::vertex::PosTex;

use crate::geometry::box_geometry;
use nalgebra_glm as glm;

pub struct CubeExample {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

impl framework::CanvasView for CubeExample {
    fn init(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self {
        use std::mem;

        let mut init_encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<PosTex>();
        let (vertex_data, index_data) = box_geometry::create_vertices();
        let vertex_buf = device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertex_data);

        let index_buf = device
            .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&index_data);

        println!("--Create the vertex and index buffers");
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
                    ty: wgpu::BindingType::SampledTexture,
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });
        println!("--Create pipeline layout");

        // Create the texture
        let (texture_view, _texture_extent, sampler) =
            texture::from_file("texture.png", device, &mut init_encoder);
        println!("--Create the texture");

        // Create other resources
        // 投影矩阵
        let radian: glm::TVec1<f32> = glm::radians(&glm::vec1(45.0));
        let p_matrix: glm::TMat4<f32> = glm::perspective_fov(
            radian[0],
            sc_desc.width as f32,
            sc_desc.height as f32,
            0.01,
            100.0,
        );
        //        let mut  p_matrix: glm::TMat4<f32> = glm::ortho(-1.0, 1.0, -1.0, 1.0, -100.0, 100.0);
        let mut vm_matrix = glm::TMat4::identity();
        vm_matrix = glm::translate(&vm_matrix, &glm::vec3(0.0, 0.0, -6.0));
        vm_matrix = glm::rotate(&vm_matrix, radian[0], &glm::vec3(1.0, 1.0, 1.0));

        let uniform_buf = device
            .create_buffer_mapped(
                16,
                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::TRANSFER_DST,
            )
            .fill_from_slice((p_matrix * vm_matrix).as_slice());

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0 .. 64,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Create the render pipeline
        let (vs_module, fs_module) = crate::shader::load_general_glsl("cube", device);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::PipelineStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::PipelineStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Cw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            },
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: vertex_size as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttributeDescriptor {
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float3,
                        offset: 0,
                    },
                    wgpu::VertexAttributeDescriptor {
                        shader_location: 1,
                        format: wgpu::VertexFormat::Float2,
                        offset: PosTex::tex_offset(),
                    },
                ],
            }],
            sample_count: 1,
        });

        // Done
        let init_command_buf = init_encoder.finish();
        device.get_queue().submit(&[init_command_buf]);
        CubeExample {
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            uniform_buf,
            pipeline,
        }
    }

    fn update(&mut self, _event: wgpu::winit::WindowEvent) {
        //empty
    }

    fn touch_moved(&mut self, _position: crate::math::Position, device: &mut wgpu::Device) {}

    fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) {
        // 投影矩阵
        let radian: glm::TVec1<f32> = glm::radians(&glm::vec1(45.0));
        let p_matrix: glm::TMat4<f32> = glm::perspective_fov(
            radian[0],
            sc_desc.width as f32,
            sc_desc.height as f32,
            0.01,
            100.0,
        );
        //        let mut  p_matrix: glm::TMat4<f32> = glm::ortho(-1.0, 1.0, -1.0, 1.0, -100.0, 100.0);
        let mut vm_matrix = glm::TMat4::identity();
        vm_matrix = glm::translate(&vm_matrix, &glm::vec3(0.0, 0.0, -6.0));
        vm_matrix = glm::rotate(&vm_matrix, radian[0], &glm::vec3(1.0, 1.0, 1.0));

        let temp_buf = device
            .create_buffer_mapped(16, wgpu::BufferUsage::TRANSFER_SRC)
            .fill_from_slice((p_matrix * vm_matrix).as_slice());

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        encoder.copy_buffer_to_buffer(&temp_buf, 0, &self.uniform_buf, 0, 64);
        device.get_queue().submit(&[encoder.finish()]);
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
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: None,
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
