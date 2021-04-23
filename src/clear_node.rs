// on metal backend, the window/view default background color is red,
// so, when use not set image, need render only background color

use idroid::geometry::plane::Plane;
use idroid::vertex::{Pos, PosTex};
use idroid::BufferObj;
use wgpu::util::DeviceExt;
use zerocopy::AsBytes;

pub struct ClearColorNode {
    vertex_buf: BufferObj,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl ClearColorNode {
    pub fn new(app_view: &idroid::AppView, device: &wgpu::Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                count: None,
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(0),
                },
            }],
        });

        let mvp_size = std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress;
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let mvp_buf = idroid::MVPUniformObj::new((&app_view.sc_desc).into(), device, &mut encoder);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            label: None,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: mvp_buf.buffer.buffer.as_entire_binding(),
            }],
        });

        // Create the vertex and index buffers
        let vertex_size = std::mem::size_of::<PosTex>();
        let (vertex_data, index_data) = Plane::new(1, 1).generate_vertices();

        let vertex_buf = BufferObj::create_buffer(
            device,
            Some(&vertex_data.as_bytes()),
            None,
            wgpu::BufferUsage::VERTEX,
            Some("vertex buffer"),
        );
        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: &index_data.as_bytes(),
            usage: wgpu::BufferUsage::INDEX,
        });

        // Create the render pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            push_constant_ranges: &[],
            bind_group_layouts: &[&bind_group_layout],
        });

        let clear_shader = idroid::shader::Shader::new("clear_color", device);
        let color_alpha_blend = idroid::utils::color_alpha_blend();
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &clear_shader.vs_module,
                entry_point: "main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<PosTex>() as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &PosTex::vertex_attributes(0),
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &clear_shader.fs_module.unwrap(),
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: app_view.sc_desc.format,
                    blend: Some(idroid::utils::default_blend()),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });
        app_view.queue.submit(Some(encoder.finish()));
        ClearColorNode {
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            pipeline,
        }
    }

    pub fn clear_color(
        &self, frame: &wgpu::SwapChainFrame, device: &mut wgpu::Device, queue: &mut wgpu::Queue,
    ) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(idroid::utils::clear_color()),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint32);
            rpass.set_vertex_buffer(0, self.vertex_buf.buffer.slice(..));
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);

            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
        }

        queue.submit(Some(encoder.finish()));
    }
}
