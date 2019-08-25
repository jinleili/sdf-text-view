use crate::geometry::plane::Plane;
use crate::texture;
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosTex};
use wgpu::Extent3d;

use nalgebra_glm as glm;

pub struct SDFRenderNode {
    extent: Extent3d,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group_outline: wgpu::BindGroup,
    bind_group_stroke: wgpu::BindGroup,
    mvp_buf: wgpu::Buffer,
    draw_buf: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DrawUniform {
    stroke_color: [f32; 4],
    mask_n_gamma: [f32; 2],
}

impl SDFRenderNode {
    pub fn new(
        sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device,
        src_view: &wgpu::TextureView, extent: Extent3d,
    ) -> Self {
        let sampler = texture::bilinear_sampler(device);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler,
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 3,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
            ],
        });
        let mvp_size = std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress;
        let mvp_buf = crate::utils::empty_uniform_buffer(device, mvp_size);

        let draw_buf = crate::utils::create_uniform_buffer(
            device,
            DrawUniform { stroke_color: [1.0, 0.0, 0.0, 1.0], mask_n_gamma: [0.063, 0.0] },
        );

        let bind_group_outline = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                    resource: wgpu::BindingResource::TextureView(src_view),
                },
                wgpu::Binding { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
                wgpu::Binding {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &draw_buf,
                        range: 0..(std::mem::size_of::<DrawUniform>() as wgpu::BufferAddress),
                    },
                },
            ],
        });

        let draw_buf_stroke = crate::utils::create_uniform_buffer(
            device,
            DrawUniform { stroke_color: [1.0, 1.0, 1.0, 1.0], mask_n_gamma: [0.25, 0.036] },
        );
        let bind_group_stroke = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                    resource: wgpu::BindingResource::TextureView(src_view),
                },
                wgpu::Binding { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
                wgpu::Binding {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &draw_buf_stroke,
                        range: 0..(std::mem::size_of::<DrawUniform>() as wgpu::BufferAddress),
                    },
                },
            ],
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
        let shader = crate::shader::Shader::new("sdf/text", device);
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let color_alpha_blend = crate::utils::color_alpha_blend();

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: shader.vertex_stage(),
            fragment_stage: shader.fragment_stage(),
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
        SDFRenderNode {
            extent,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group_outline,
            bind_group_stroke,
            pipeline,
            mvp_buf,
            draw_buf,
        }
    }

    pub fn update_scale(
        &mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device, scale: f32,
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

        let mvp: [[f32; 4]; 4] = (p_matrix * vm_matrix).into();
        crate::utils::update_uniform(device, mvp, &self.mvp_buf);
    }

    pub fn begin_render_pass(
        &self, frame: &wgpu::SwapChainOutput, encoder: &mut wgpu::CommandEncoder,
        device: &mut wgpu::Device,
    ) {
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
        rpass.set_bind_group(0, &self.bind_group_outline, &[]);
        rpass.set_index_buffer(&self.index_buf, 0);
        rpass.set_vertex_buffers(0, &[(&self.vertex_buf, 0)]);
        rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);

        // Need use update_uniform to improve
        rpass.set_bind_group(0, &self.bind_group_stroke, &[]);
        rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);

        // crate::utils::update_uniform(device,  DrawUniform { stroke_color: [1.0, 0.0, 0.0, 1.0], mask_n_gamma: [0.30, 0.0], } , &self.draw_buf);

        // crate::utils::update_uniform(device,  DrawUniform { stroke_color: [1.0, 1.0, 1.0, 1.0], mask_n_gamma: [0.25, 0.145], } , &self.draw_buf);
        // rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
    }
}