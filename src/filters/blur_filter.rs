use crate::geometry::plane::Plane;
use crate::texture;
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosTex};
use crate::SurfaceView;

use uni_view::{AppView, GPUContext};

use nalgebra_glm as glm;

use std::rc::Rc;

use crate::node::NoneNode;

#[repr(C)]
#[derive(Clone, Copy)]
struct BlurUniform {
    uv_step: f32,
    is_direction_x: f32,
}

pub struct BlurNode {
    vertex_buf: Rc<wgpu::Buffer>,
    index_buf: Rc<wgpu::Buffer>,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    pipeline: Rc<wgpu::RenderPipeline>,
    target_view: Rc<wgpu::TextureView>,
}

pub struct BlurFilter {
    app_view: AppView,
    vertex_buf: Rc<wgpu::Buffer>,
    index_buf: Rc<wgpu::Buffer>,
    index_count: usize,
    uniform_buf: wgpu::Buffer,
    blur_nodes: [BlurNode; 3],
    final_node: NoneNode,
}

impl BlurNode {
    pub fn render_pass(&self, frame: &wgpu::SwapChainOutput, encoder: &mut wgpu::CommandEncoder) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &self.target_view,
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
    }
}

fn generate_uniforms(sc_desc: &wgpu::SwapChainDescriptor) -> MVPUniform {
    //
    let radian: glm::TVec1<f32> = glm::radians(&glm::vec1(75.0));
    let p_matrix: glm::TMat4<f32> =
        glm::perspective_fov(radian[0], sc_desc.width as f32, sc_desc.height as f32, 0.01, 100.0);
    //        let mut  p_matrix: glm::TMat4<f32> = glm::ortho(-1.0, 1.0, -1.0, 1.0, -100.0, 100.0);
    let mut vm_matrix = glm::TMat4::identity();
    vm_matrix = glm::translate(&vm_matrix, &glm::vec3(0.0, 0.0, -2.12));
    // vm_matrix = glm::scale(&vm_matrix, &glm::vec3(1.0, 2.0, 2.0));
    // vm_matrix = glm::rotate(&vm_matrix, radian[0], &glm::vec3(0.0, 1.0, 0.0));
    MVPUniform { mvp_matrix: (p_matrix * vm_matrix).into() }
    // 这个步长大，模糊的效果也明显，但如果超过 4 倍像素步长，则看起来像重影了
    // uv_step: 1.0 / 512.0,
    // direction_x: 1.0,
}

fn generate_bind_group(
    device: &wgpu::Device, layout: &wgpu::BindGroupLayout, uniform_buf: &wgpu::Buffer,
    blur_buf: &wgpu::Buffer, texture_view: &wgpu::TextureView, sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: layout,
        bindings: &[
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: uniform_buf,
                    range: 0..(std::mem::size_of::<MVPUniform>() as wgpu::BufferAddress),
                },
            },
            wgpu::Binding {
                binding: 1,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &blur_buf,
                    range: 0..(std::mem::size_of::<BlurUniform>() as wgpu::BufferAddress),
                },
            },
            wgpu::Binding {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(texture_view),
            },
            wgpu::Binding { binding: 3, resource: wgpu::BindingResource::Sampler(sampler) },
        ],
    })
}

impl BlurFilter {
    pub fn new(app_view: AppView) -> Self {
        use std::mem;
        let mut app_view = app_view;

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
                        visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::UniformBuffer,
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 2,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture,
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 3,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler,
                    },
                ],
            });

        // Create the texture
        let (texture_view, _texture_extent, sampler) =
            texture::from_file("iphone.png", &mut app_view.device, &mut encoder);

        let x_blur_view = Rc::new(crate::texture::empty(
            &mut app_view.device,
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::Extent3d { width: 200, height: 200, depth: 1 },
        ));
        let xy_blur_view = Rc::new(crate::texture::empty(
            &mut app_view.device,
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::Extent3d { width: 200, height: 200, depth: 1 },
        ));

        // Create other resources
        let uniform_buf = crate::utils::create_uniform_buffer(
            &mut app_view.device,
            generate_uniforms(&app_view.sc_desc),
        );
        let mvp = MVPUniform { mvp_matrix: crate::matrix_helper::ortho_default_mvp() };
        let mvp_buf0 = crate::utils::create_uniform_buffer(&mut app_view.device, mvp);

        let x_blur_buf = crate::utils::create_uniform_buffer(
            &mut app_view.device,
            BlurUniform { uv_step: 1.0 / 1000.0, is_direction_x: 1.0 },
        );
        let y_blur_buf = crate::utils::create_uniform_buffer(
            &mut app_view.device,
            BlurUniform { uv_step: 1.0 / 1000.0, is_direction_x: 0.0 },
        );

        // Create bind group
        let bind_group0 = generate_bind_group(
            &mut app_view.device,
            &bind_group_layout,
            &mvp_buf0,
            &x_blur_buf,
            &texture_view,
            &sampler,
        );
        let bind_group1 = generate_bind_group(
            &mut app_view.device,
            &bind_group_layout,
            &mvp_buf0,
            &y_blur_buf,
            &x_blur_view,
            &sampler,
        );
        let bind_group2 = generate_bind_group(
            &mut app_view.device,
            &bind_group_layout,
            &mvp_buf0,
            &x_blur_buf,
            &xy_blur_view,
            &sampler,
        );
        println!("--create_bind_group");

        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<PosTex>();
        let (vertex_data, index_data) = Plane::new(1, 1).generate_vertices();
        let vertex_buf = Rc::new(
            app_view
                .device
                .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
                .fill_from_slice(&vertex_data),
        );

        let index_buf = Rc::new(
            app_view
                .device
                .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
                .fill_from_slice(&index_data),
        );

        // Create the render pipeline
        let shader = crate::shader::Shader::new("filter/gaussian_blur", &mut app_view.device);
        let pipeline_layout =
            app_view.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            });
        println!("--Create pipeline layout: {:?}", app_view.sc_desc.format);
        let pipeline =
            Rc::new(app_view.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                depth_stencil_state: None,
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[wgpu::VertexBufferDescriptor {
                    stride: vertex_size as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &PosTex::attri_descriptor(0),
                }],
                sample_count: 1,
            }));

        let blur_nodes = [
            BlurNode {
                vertex_buf: vertex_buf.clone(),
                index_buf: index_buf.clone(),
                index_count: index_data.len(),
                bind_group: bind_group0,
                pipeline: pipeline.clone(),
                target_view: x_blur_view.clone(),
            },
            BlurNode {
                vertex_buf: vertex_buf.clone(),
                index_buf: index_buf.clone(),
                index_count: index_data.len(),
                bind_group: bind_group1,
                pipeline: pipeline.clone(),
                target_view: xy_blur_view.clone(),
            },
            BlurNode {
                vertex_buf: vertex_buf.clone(),
                index_buf: index_buf.clone(),
                index_count: index_data.len(),
                bind_group: bind_group2,
                pipeline: pipeline.clone(),
                target_view: x_blur_view.clone(),
            },
        ];

        // Done
        let init_command_buf = encoder.finish();
        app_view.device.get_queue().submit(&[init_command_buf]);

        let mvp = crate::matrix_helper::default_mvp(&app_view.sc_desc);
        let final_node = NoneNode::new(
            &app_view.sc_desc,
            &mut app_view.device,
            &xy_blur_view,
            MVPUniform { mvp_matrix: mvp },
        );
        BlurFilter {
            app_view,
            vertex_buf: vertex_buf.clone(),
            index_buf: index_buf.clone(),
            index_count: index_data.len(),
            uniform_buf,
            blur_nodes,
            final_node,
        }
    }
}
impl SurfaceView for BlurFilter {
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
    }

    fn enter_frame(&mut self) {
        let mut encoder = self
            .app_view
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let frame = self.app_view.swap_chain.get_next_texture();
            {
                let node = &self.blur_nodes[0];
                node.render_pass(&frame, &mut encoder);
                let xy_node = &self.blur_nodes[1];
                let x_node = &self.blur_nodes[2];
                xy_node.render_pass(&frame, &mut encoder);

                for _ in 0..=2 {
                    x_node.render_pass(&frame, &mut encoder);
                    xy_node.render_pass(&frame, &mut encoder);
                }
            }
            {
                self.final_node.begin_render_pass(&frame, &mut encoder);
            }

            self.app_view.device.get_queue().submit(&[encoder.finish()]);
        }
    }
}
