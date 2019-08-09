use crate::geometry::plane::Plane;
use crate::utils::{create_uniform_buffer, MVPUniform};
use crate::vertex::{Pos, PosTex};

pub struct PaperNode {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    mvp_buf: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

impl PaperNode {
    pub fn new(
        sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device,
        src_view: Option<&wgpu::TextureView>, shader_name: &str,
    ) -> Self {
        let mut bgl_indings: Vec<wgpu::BindGroupLayoutBinding> =
            vec![wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer,
            }];

        let mvp = crate::matrix_helper::ortho_default_mvp();
        let mvp_buf = create_uniform_buffer(device, mvp);

        let mut bindings: Vec<wgpu::Binding> = vec![wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::Buffer {
                buffer: &mvp_buf,
                range: 0..(std::mem::size_of::<MVPUniform>() as wgpu::BufferAddress),
            },
        }];

        let sampler = crate::texture::default_sampler(device);

        if let Some(texture_view) = src_view {
            bgl_indings.push(wgpu::BindGroupLayoutBinding {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture,
            });
            bgl_indings.push(wgpu::BindGroupLayoutBinding {
                binding: 2,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler,
            });

            bindings.push(wgpu::Binding {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            });
            bindings.push(wgpu::Binding {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&sampler),
            });
        }

        let bind_group_layout = device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { bindings: &bgl_indings });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &bindings,
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
        let shader = crate::shader::Shader::new(shader_name, device);
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });
        let color_alpha_blend = crate::utils::color_alpha_blend();

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
                color_blend: color_alpha_blend.0,
                alpha_blend: color_alpha_blend.1,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            // ??????
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: vertex_size as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &PosTex::attri_descriptor(0),
            }],
            sample_count: 1,
        });

        PaperNode {
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            pipeline,
            mvp_buf,
        }
    }

    pub fn begin_render_pass(
        &self, target_view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder,
        load_op: wgpu::LoadOp,
    ) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: target_view,
                resolve_target: None,
                load_op: load_op,
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
    }
}
