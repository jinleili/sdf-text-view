use crate::math::{Position, Rect, ViewSize};
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosBrush};

use crate::geometry::Line;

use nalgebra_glm as glm;

pub struct ParticleNode {
    view_size: ViewSize,
    vertex_buf: wgpu::Buffer,
    vertex_count: usize,
    bind_group: wgpu::BindGroup,
    mvp_buf: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    pub particle_position_tv: wgpu::TextureView,
    depth_texture_view: wgpu::TextureView,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ParticleUniform {
    // lattice 在正规化坐标空间的大小
    lattice_size: [f32; 2],
    // 粒子数
    particle_nxy: [f32; 2],
}

impl ParticleNode {
    pub fn new(
        sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device, mvp: MVPUniform,
        particle_nx: u32, particle_ny: u32, param_buffer: &wgpu::Buffer,
    ) -> Self {
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
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::SampledTexture,
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 2,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Sampler,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let mvp_buf = crate::utils::create_uniform_buffer(device, mvp);

        let view_size = ViewSize { width: sc_desc.width as f32, height: sc_desc.height as f32 };

        let (particle_position_tv, _, _) = crate::texture::from_buffer_and_usage_write(
            param_buffer,
            device,
            &mut encoder,
            particle_nx,
            particle_ny,
            4 * 4,
            true,
        );
        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                    resource: wgpu::BindingResource::TextureView(&particle_position_tv),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&crate::texture::bilinear_sampler(
                        device,
                    )),
                },
            ],
        });

        // Create the vertex and index buffers
        let vertex_data = self::particle_vertex_data(particle_nx, particle_ny);
        let vertex_size = mem::size_of::<[f32; 2]>();
        let vertex_buf = device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertex_data);

        // Create the render pipeline
        let shader = crate::shader::Shader::new("fluid2/particle_rendering", device);
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
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: vertex_size as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &vec![wgpu::VertexAttributeDescriptor {
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float2,
                    offset: 0,
                }],
            }],
            sample_count: 1,
        });

        let depth_texture_view = crate::depth_stencil::create_depth_texture_view(sc_desc, device);
        {
            // Create pipeline layout
            let bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    bindings: &[
                        wgpu::BindGroupLayoutBinding {
                            binding: 0,
                            visibility: wgpu::ShaderStage::COMPUTE,
                            ty: wgpu::BindingType::UniformBuffer,
                        },
                        wgpu::BindGroupLayoutBinding {
                            binding: 1,
                            visibility: wgpu::ShaderStage::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture,
                        },
                    ],
                });

            let uniform = ParticleUniform {
                lattice_size: [2.0 / 100.0, 2.0 / 100.0],
                particle_nxy: [particle_nx as f32, particle_ny as f32],
            };
            let uniform_buf = crate::utils::create_uniform_buffer(device, uniform);
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &uniform_buf,
                            range: 0..(std::mem::size_of::<ParticleUniform>()
                                as wgpu::BufferAddress),
                        },
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&particle_position_tv),
                    },
                ],
            });
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            });
            let shader = crate::shader::Shader::new_by_compute("fluid2/particle_init", device);
            let compute_pipeline =
                device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    layout: &pipeline_layout,
                    compute_stage: shader.cs_stage(),
                });

            let mut cpass = encoder.begin_compute_pass();
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.dispatch(particle_nx, particle_ny, 1);
        }
        device.get_queue().submit(&[encoder.finish()]);

        ParticleNode {
            view_size,
            vertex_buf,
            vertex_count: vertex_data.len(),
            bind_group,
            mvp_buf,
            pipeline,
            particle_position_tv,
            depth_texture_view,
        }
    }

    pub fn begin_render_pass(&self, frame: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame,
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
        rpass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
        rpass.draw(0..self.vertex_count as u32, 0..1);
        // rpass.draw(0 .. 6 , 0 .. 1);
    }
}

pub fn particle_vertex_data(nx: u32, ny: u32) -> Vec<[f32; 2]> {
    let mut list = vec![];
    let step_x: f32 = 1.0 / (nx as f32);
    let step_y: f32 = 1.0 / (ny as f32);
    for i in 0..nx {
        for j in 0..ny {
            list.push([i as f32 * step_x, j as f32 * step_y]);
        }
    }
    list
}
