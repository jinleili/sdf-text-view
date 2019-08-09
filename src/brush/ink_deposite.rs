use crate::math::{Position, Rect, ViewSize};
use crate::texture;
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosBrush, PosWeight};

use nalgebra_glm as glm;

use std::rc::Rc;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct LineUniform {
    blur_distance: f32,
    edge_distance: f32,
    lookup_table: [f32; 32],
}

pub struct InkDeposite {
    view_size: ViewSize,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    vertex_count: usize,
    bind_group: wgpu::BindGroup,
    mvp_buf: wgpu::Buffer,
    buf: wgpu::Buffer,
    uniform: LineUniform,
    pipeline: wgpu::RenderPipeline,
    depth_texture_view: wgpu::TextureView,
    pub canvas_texture_view: Rc<wgpu::TextureView>,
    point_node: crate::node::PointNode,
    control_point_node: crate::node::PointNode,
    // stroke 计算
    stroke_calculator: crate::brush::StrokeCalculator,
    is_animating: bool,
    is_touch_moving: bool,
    touch_start: Position,
    last_touch: Position,
    rotate: f32,
}

fn generate_mv_matrix(translate_z: f32) -> glm::TMat4<f32> {
    let mut vm_matrix = glm::TMat4::identity();
    vm_matrix = glm::translate(&vm_matrix, &glm::vec3(0.0, 0.0, translate_z));
    // let radian: glm::TVec1<f32> = glm::radians(&glm::vec1(95.0));
    // vm_matrix = glm::rotate(&vm_matrix, radian[0], &glm::vec3(0.0, 0.0, 1.0));
    vm_matrix
}

impl InkDeposite {
    pub fn new(
        sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Self {
        use std::mem;
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
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let p_matrix: glm::TMat4<f32> =
            crate::matrix_helper::ortho_pixel(sc_desc.width as f32, sc_desc.height as f32);
        let mvp_uniform = MVPUniform { mvp_matrix: (p_matrix * generate_mv_matrix(-100.0)).into() };
        let mvp_buf = crate::utils::create_uniform_buffer(device, mvp_uniform);

        let view_size = ViewSize { width: sc_desc.width as f32, height: sc_desc.height as f32 };

        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<PosBrush>();
        // let (vertex_data, index_data) = curve.generate_vertices(120, 0);
        let (vertex_data, index_data, control_vertex) = crate::brush::generate_vertices(80);
        let vertex_buf = device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertex_data);

        let index_buf = device
            .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&index_data);

        let uniform = LineUniform {
            blur_distance: 0.0,
            edge_distance: 0.0,
            lookup_table: crate::utils::gaussian::lookup_table(),
        };
        let buf = crate::utils::create_uniform_buffer(device, uniform);

        let (texture_view, _texture_extent, sampler) =
            // texture::from_file("pencil2.png", device, encoder);
            texture::from_file("brush/brush4.png", device, encoder);
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
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &buf,
                        range: 0..(std::mem::size_of::<LineUniform>() as wgpu::BufferAddress),
                    },
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::Binding { binding: 3, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });

        // Create the render pipeline
        let shader = crate::shader::Shader::new("brush/brush", device);
        let color_alpha_blend = crate::utils::color_blend_subtract();
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
            // depth_stencil_state: None,
            depth_stencil_state: Some(crate::depth_stencil::create_state_descriptor()),
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: vertex_size as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &PosBrush::attri_descriptor(0),
            }],
            sample_count: 1,
        });

        // 空的画布
        let canvas_texture_view =
            Rc::new(crate::texture::empty_view(device, sc_desc.width, sc_desc.height));
        let paper_node = super::PaperNode::new(sc_desc, device, None, "brush/empty_canvas");
        paper_node.begin_render_pass(&canvas_texture_view, encoder, wgpu::LoadOp::Clear);

        let depth_texture_view = crate::depth_stencil::create_depth_texture_view(sc_desc, device);

        let points_data = super::get_touchpoints();
        let point_node = crate::node::PointNode::new(sc_desc, device, mvp_uniform, &points_data);
        let control_point_node =
            crate::node::PointNode::new(sc_desc, device, mvp_uniform, &control_vertex);

        let stroke_calculator = crate::brush::StrokeCalculator::new();

        InkDeposite {
            view_size,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            vertex_count: vertex_data.len(),
            bind_group,
            mvp_buf,
            buf,
            uniform,
            pipeline,
            depth_texture_view,
            canvas_texture_view,
            point_node,
            control_point_node,
            stroke_calculator,
            is_animating: false,
            is_touch_moving: false,
            touch_start: Position::zero(),
            last_touch: Position::zero(),
            rotate: 0.0,
        }
    }

    pub fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) {
        self.view_size = ViewSize { width: sc_desc.width as f32, height: sc_desc.height as f32 };
        // crate::utils::update_uniform(device, generate_uniforms(sc_desc), &self.uniform_buf);
        self.depth_texture_view = crate::depth_stencil::create_depth_texture_view(sc_desc, device);
    }

    pub fn begin_render_pass(&self, encoder: &mut wgpu::CommandEncoder) {
        if self.index_count > 0 {
            // 做 blend 时，抵消了上一步的一样绘制点？
            // 测试结果：
            // 与 z 冲突无关
            // 与开启 depth_stencil_attachment 无关
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.canvas_texture_view,
                    resolve_target: Some(&self.canvas_texture_view),
                    load_op: wgpu::LoadOp::Load,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: crate::utils::clear_color(),
                    // clear_color: wgpu::Color::TRANSPARENT,
                }],
                // depth_stencil_attachment: None,
                depth_stencil_attachment: Some(crate::depth_stencil::create_attachment_descriptor(
                    &self.depth_texture_view,
                )),
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
            rpass.set_index_buffer(&self.index_buf, 0);
            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);

            // 顺序发生变化的位置
            // rpass.draw_indexed(0 .. 6, 0, 0 .. 1);
            // rpass.draw_indexed(6 .. 12, 0, 0 .. 1);
            // rpass.draw_indexed(12 .. 18, 0, 0 .. 1);
        };
        {
            // 绘制顶点
            self.point_node.begin_render_pass(&self.canvas_texture_view, encoder);
        }
        {
            self.control_point_node.begin_render_pass(&self.canvas_texture_view, encoder);
        }
    }
}

impl InkDeposite {
    pub fn touch_start(&mut self, device: &mut wgpu::Device, p: [f32; 2]) {
        let (vertex_data, index_data, control_vertex) = self.stroke_calculator.path_start(p);
        self.update_buf(vertex_data, index_data, device);
    }

    pub fn touch_moved(&mut self, device: &mut wgpu::Device, p: [f32; 2]) {
        let (vertex_data, index_data, control_vertex) = self.stroke_calculator.path_move_linear(p);
        self.update_buf(vertex_data, index_data, device);
    }

    pub fn touch_end(&mut self, device: &mut wgpu::Device, p: [f32; 2]) {
        let (vertex_data, index_data, control_vertex) = self.stroke_calculator.path_end(p);
        self.update_buf(vertex_data, index_data, device);
    }

    fn update_buf(
        &mut self, vertex_data: Vec<PosBrush>, index_data: Vec<u16>, device: &mut wgpu::Device,
    ) {
        self.index_count = index_data.len();
        // 避免 iOS 上 failed assertion `indexCount(0) must be non-zero.'
        if self.index_count > 0 {
            self.vertex_buf = device
                .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
                .fill_from_slice(&vertex_data);

            self.index_buf = device
                .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
                .fill_from_slice(&index_data);
        }
    }

    // 动画帧插值
    fn interpolate_frame_data(&mut self, target_pos: Position, is_open: bool) {}

    fn fresh_frame_data(&mut self, device: &mut wgpu::Device) {}

    fn step_frame_data(&mut self, position: Position, device: &mut wgpu::Device) {}
}
