// on metal backend, the window/view default background color is red,
// so, when use not set image, need render only background color

use idroid::geometry::plane::Plane;
use idroid::utils::MVPUniform;
use idroid::vertex::{Pos, PosTex};

use nalgebra_glm as glm;

pub struct ClearColorNode {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl ClearColorNode {
    pub fn new(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
        });

        let mvp_size = std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress;
        let mvp_buf = idroid::utils::empty_uniform_buffer(device, mvp_size);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer { buffer: &mvp_buf, range: 0..mvp_size },
            }],
        });

        // Create the vertex and index buffers
        let vertex_size = std::mem::size_of::<PosTex>();
        let (vertex_data, index_data) = Plane::new(1, 1).generate_vertices();

        let vertex_buf = device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertex_data);

        let index_buf = device
            .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&index_data);
        // Create the render pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let clear_shader = idroid::shader::Shader::new("clear_color", device, env!("CARGO_MANIFEST_DIR"));
        let color_alpha_blend = idroid::utils::color_alpha_blend();
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: clear_shader.vertex_stage(),
            fragment_stage: clear_shader.fragment_stage(),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: color_alpha_blend.0,
                alpha_blend: color_alpha_blend.1,
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
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });
        ClearColorNode {
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            pipeline,
        }
    }

    pub fn clear_color(&self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: idroid::utils::clear_color(),
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_index_buffer(&self.index_buf, 0);
            rpass.set_vertex_buffers(0, &[(&self.vertex_buf, 0)]);
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);

            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
        }

        device.get_queue().submit(&[encoder.finish()]);
    }
}
