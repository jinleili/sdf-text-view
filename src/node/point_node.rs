use crate::math::{ViewSize};
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosBrush};

#[allow(dead_code)]
pub struct PointNode {
    view_size: ViewSize,
    vertex_buf: wgpu::Buffer,
    vertex_count: usize,
    bind_group: wgpu::BindGroup,
    mvp_buf: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    depth_texture_view: wgpu::TextureView,
}

#[allow(dead_code)]
impl PointNode {
    pub fn new(
        sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device, mvp: MVPUniform,
        vertex_data: &Vec<PosBrush>,
    ) -> Self {
        use std::mem;
        let _encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        // Create pipeline layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer,
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let mvp_buf = crate::utils::create_uniform_buffer(device, mvp);

        let view_size = ViewSize { width: sc_desc.width as f32, height: sc_desc.height as f32 };

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &mvp_buf,
                    range: 0..(std::mem::size_of::<MVPUniform>() as wgpu::BufferAddress),
                },
            }],
        });

        // Create the vertex and index buffers
        // let vertex_size = mem::size_of::<PosWeight>();
        let vertex_size = mem::size_of::<PosBrush>();
        // let mut new_data: Vec<PosWeight> = vec![];
        // for i in 0..vertex_data.len() / 2 {
        //     new_data.push(vertex_data[i]);
        // }
        let vertex_buf = device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(vertex_data);

        // Create the render pipeline
        let shader = crate::shader::Shader::new("node/point", device);
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
            primitive_topology: wgpu::PrimitiveTopology::PointList,
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
            index_format: wgpu::IndexFormat::Uint32,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: vertex_size as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &PosBrush::attri_descriptor(0),
            }],
            sample_count: 1,
        });

        let depth_texture_view = crate::depth_stencil::create_depth_texture_view(sc_desc, device);

        PointNode {
            view_size,
            vertex_buf,
            vertex_count: vertex_data.len(),
            bind_group,
            mvp_buf,
            pipeline,
            depth_texture_view,
        }
    }

    pub fn begin_render_pass(&self, frame: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame,
                resolve_target: None,
                load_op: wgpu::LoadOp::Load,
                store_op: wgpu::StoreOp::Store,
                clear_color: crate::utils::clear_color(),
            }],
            depth_stencil_attachment: Some(crate::depth_stencil::create_attachment_descriptor(
                &self.depth_texture_view,
            )),
        });
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
        rpass.draw(0..self.vertex_count as u32, 0..1);
        // rpass.draw(0 .. 6 , 0 .. 1);
    }
}
