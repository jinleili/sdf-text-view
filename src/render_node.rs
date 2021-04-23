use idroid::geometry::plane::Plane;
use idroid::vertex::{Pos, PosTex};
use idroid::{texture, BufferObj, MVPUniform, MVPUniformObj};

use nalgebra_glm as glm;
use wgpu::util::DeviceExt;
use wgpu::Extent3d;
use zerocopy::{AsBytes, FromBytes};

pub struct SDFRenderNode {
    extent: Extent3d,
    scale: f32,
    vertex_buf: BufferObj,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group_outline: wgpu::BindGroup,
    bind_group_stroke: wgpu::BindGroup,
    mvp_buf: MVPUniformObj,
    pipeline: wgpu::RenderPipeline,
}

#[repr(C)]
#[derive(Clone, Copy, AsBytes, FromBytes)]
pub struct DrawUniform {
    stroke_color: [f32; 4],
    mask_n_gamma: [f32; 2],
}

impl SDFRenderNode {
    pub fn new(
        app_view: &idroid::AppView, device: &wgpu::Device, src_view: &wgpu::TextureView,
        extent: Extent3d,
    ) -> Self {
        let sampler = texture::bilinear_sampler(device);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    count: None,
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(0),
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    count: None,
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    count: None,
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false, filtering: true },
                },
                wgpu::BindGroupLayoutEntry {
                    count: None,
                    binding: 3,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(0),
                    },
                },
            ],
        });
        let mvp_size = std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress;
 let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let mvp_buf = idroid::MVPUniformObj::new((&app_view.sc_desc).into(), device, &mut encoder);

        let draw_buf = idroid::BufferObj::create_uniform_buffer(
            device,
            &DrawUniform { stroke_color: [0.14, 0.14, 0.14, 1.0], mask_n_gamma: [0.70, 0.0] },
        );

        let bind_group_outline = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            label: None,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: mvp_buf.buffer.buffer.as_entire_binding() },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(src_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry { binding: 3, resource: draw_buf.buffer.as_entire_binding() },
            ],
        });

        let draw_buf_stroke = BufferObj::create_uniform_buffer(
            device,
            &DrawUniform { stroke_color: [0.97, 0.92, 0.80, 1.0], mask_n_gamma: [0.75, 0.75] },
        );
        let bind_group_stroke = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            label: None,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: mvp_buf.buffer.buffer.as_entire_binding() },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(src_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry { binding: 3, resource: draw_buf_stroke.buffer.as_entire_binding() },
            ],
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
        let shader = idroid::shader::Shader::new("sdf/text", device);
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            push_constant_ranges: &[],
            bind_group_layouts: &[&bind_group_layout],
        });

        let color_alpha_blend = idroid::utils::color_alpha_blend();
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader.vs_module,
                entry_point: "main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<PosTex>() as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &PosTex::vertex_attributes(0),
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader.fs_module.unwrap(),
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
        SDFRenderNode {
            extent,
            scale: 1.0,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group_outline,
            bind_group_stroke,
            pipeline,
            mvp_buf,
        }
    }

    pub fn update_scale(
        &mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device,
        encoder: &mut wgpu::CommandEncoder, scale: f32,
    ) {
        let fovy: f32 = 75.0 / 180.0 * std::f32::consts::PI;
        let radian: glm::TVec1<f32> = glm::vec1(fovy);
        let p_matrix: glm::TMat4<f32> = glm::perspective_fov(
            radian[0],
            sc_desc.width as f32,
            sc_desc.height as f32,
            0.1,
            100.0,
        );
        let mut vm_matrix = glm::TMat4::identity();
        let sc_ratio = sc_desc.width as f32 / sc_desc.height as f32;
        let tex_ratio = self.extent.width as f32 / self.extent.height as f32;
        // maintain texture's aspect ratio
        vm_matrix = glm::scale(&vm_matrix, &glm::vec3(1.0, 1.0 / tex_ratio, 1.0));

        // when viewport's h > w,  ratio = h / w, when w > h ，ratio = 1
        let ratio = if sc_ratio < 1.0 { sc_desc.height as f32 / sc_desc.width as f32 } else { 1.0 };
        // use fovy calculate z translate distance
        let factor: f32 = (fovy / 2.0).tan();

        // full fill viewport's width or height
        let mut translate_z = -(ratio / factor);
        if sc_ratio < tex_ratio {
            if tex_ratio > 1.0 {
                translate_z /= sc_ratio * ratio;
            }
        } else {
            translate_z /= tex_ratio;
            // when tex h > w and viewport h > w, need fill the viewport's height, and the height ration is not 1.0
            if tex_ratio < 1.0 {
                translate_z /= ratio;
            };
        }
        vm_matrix = glm::translate(&vm_matrix, &glm::vec3(0.0, 0.0, translate_z));

        self.scale *= scale;
        vm_matrix = glm::scale(&vm_matrix, &glm::vec3(self.scale, self.scale, 1.0));

        let mvp: [[f32; 4]; 4] = (p_matrix * vm_matrix).into();
        self.mvp_buf.buffer.update_buffer(encoder, device, &mvp);
    }

    pub fn begin_render_pass(
        &self, frame: &wgpu::SwapChainFrame, encoder: &mut wgpu::CommandEncoder,
    ) {
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

        rpass.set_bind_group(0, &self.bind_group_outline, &[]);
        rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);

        // Need use update_uniform to improve
        rpass.set_bind_group(0, &self.bind_group_stroke, &[]);
        rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
    }
}
