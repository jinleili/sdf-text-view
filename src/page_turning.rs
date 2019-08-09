use crate::math::{Position, Rect, ViewSize};
use crate::texture;
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosTex};

use crate::geometry::plane::Plane;
use crate::node::NoneNode;
use crate::SurfaceView;

use uni_view::{AppView, GPUContext};

use nalgebra_glm as glm;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct TurningUniform {
    radius: f32,
    angle: f32,
    np: [f32; 2],
    n: [f32; 2],
    alpha: f32,
}

pub struct PageTurning {
    app_view: AppView,
    page_rect: Rect,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    mvp_buf: wgpu::Buffer,
    turning_buf: wgpu::Buffer,
    turning_uniform: TurningUniform,
    pipeline: wgpu::RenderPipeline,
    depth_texture_view: wgpu::TextureView,
    bottom_node: NoneNode,
    is_animating: bool,
    is_touch_moving: bool,
    touch_start: Position,
    last_touch: Position,
    roll_length: f32,
    frame_data: Vec<Position>,
    if_need_stop: bool, // 是否停止动画渲染
}

fn generate_mv_matrix(translate_z: f32) -> glm::TMat4<f32> {
    let mut vm_matrix = glm::TMat4::identity();
    vm_matrix = glm::translate(&vm_matrix, &glm::vec3(0.0, 0.0, translate_z));
    vm_matrix
}

impl PageTurning {
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
        let pipeline_layout =
            app_view.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            });

        // Create the texture
        let (texture_view, _texture_extent, sampler) =
            texture::from_file("page_turning.png", &mut app_view.device, &mut encoder);

        let p_matrix: glm::TMat4<f32> = crate::matrix_helper::ortho_pixel(
            app_view.sc_desc.width as f32,
            app_view.sc_desc.height as f32,
        );
        let mvp_uniform = MVPUniform { mvp_matrix: (p_matrix * generate_mv_matrix(-400.0)).into() };
        let mvp_buf = crate::utils::create_uniform_buffer(&mut app_view.device, mvp_uniform);

        let view_size = ViewSize {
            width: app_view.sc_desc.width as f32,
            height: app_view.sc_desc.height as f32,
        };
        let page_rect =
            Rect::new(375.0 * app_view.scale_factor, 667.0 * app_view.scale_factor, view_size);

        let turning_uniform = TurningUniform {
            radius: page_rect.width / 8.0,
            angle: 0.0,
            alpha: 1.0,
            np: [0.0, 0.0],
            n: [0.0, 0.0],
        };
        let turning_buf =
            crate::utils::create_uniform_buffer(&mut app_view.device, turning_uniform);

        // Create bind group
        let bind_group = app_view.device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                        buffer: &turning_buf,
                        range: 0..(std::mem::size_of::<TurningUniform>() as wgpu::BufferAddress),
                    },
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::Binding { binding: 3, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });

        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<PosTex>();
        let (vertex_data, index_data) =
            Plane::new_by_pixel(page_rect.width, page_rect.height, 200, 300).generate_vertices();
        let vertex_buf = app_view
            .device
            .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertex_data);

        let index_buf = app_view
            .device
            .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&index_data);

        // Create the render pipeline
        let color_alpha_blend = crate::utils::color_alpha_blend();
        let shader = crate::shader::Shader::new("page_turning", &mut app_view.device);
        let pipeline = app_view.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                attributes: &PosTex::attri_descriptor(0),
            }],
            sample_count: 1,
        });

        // Done
        let init_command_buf = encoder.finish();
        app_view.device.get_queue().submit(&[init_command_buf]);

        let depth_texture_view = crate::depth_stencil::create_depth_texture_view(
            &app_view.sc_desc,
            &mut app_view.device,
        );
        let mut vm_matrix = generate_mv_matrix(-500.0);
        vm_matrix =
            glm::scale(&vm_matrix, &glm::vec3(page_rect.width / 2.0, page_rect.height / 2.0, 1.0));

        let bottom_node = NoneNode::new(
            &app_view.sc_desc,
            &mut app_view.device,
            &texture_view,
            MVPUniform { mvp_matrix: (p_matrix * vm_matrix).into() },
        );

        PageTurning {
            app_view,
            page_rect,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            bind_group,
            mvp_buf,
            turning_buf,
            turning_uniform,
            pipeline,
            depth_texture_view,
            bottom_node,
            is_animating: false,
            is_touch_moving: false,
            touch_start: Position::zero(),
            last_touch: Position::zero(),
            frame_data: vec![],
            roll_length: 0.0,
            if_need_stop: false,
        }
    }
}

impl SurfaceView for PageTurning {
    fn update(&mut self, _event: wgpu::winit::WindowEvent) {
        //empty
    }

    fn touch_moved(&mut self, position: Position) {
        println!("touch_moved--");
        if self.is_animating {
            self.is_touch_moving = false;
            println!("touch_ return--");
            return;
        }
        // 转换成正交坐标
        let position = position.multiply_f(self.app_view.scale_factor).ortho_in(ViewSize {
            width: self.app_view.sc_desc.width as f32,
            height: self.app_view.sc_desc.height as f32,
        });
        let x_right = self.page_rect.center_x();

        if self.is_touch_moving && !self.page_rect.is_ortho_intersect(position) {
            self.is_touch_moving = false;
            self.is_animating = true;
            self.touch_end();
            return;
        }

        self.if_need_stop = false;
        if self.touch_start.is_equal_zero() {
            if self.page_rect.is_ortho_intersect(position) && position.x >= (x_right - 15.0) {
                self.touch_start = position;
                self.is_touch_moving = true;
                self.roll_length = 0.0;
                println!("touch_start--");
            }
        } else {
            println!("touch_moving--");
            self.is_touch_moving = true;
            self.step_frame_data(position);
        }
        self.last_touch = position;
    }

    fn resize(&mut self) {
        // crate::utils::update_uniform(device, generate_uniforms(sc_desc), &self.uniform_buf);
        self.depth_texture_view = crate::depth_stencil::create_depth_texture_view(
            &self.app_view.sc_desc,
            &mut self.app_view.device,
        );
        self.if_need_stop = false;
    }

    fn enter_frame(&mut self) {
        // if self.if_need_stop {
        //     return;
        // }

        self.fresh_frame_data();

        let mut encoder = self
            .app_view
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let frame = self.app_view.swap_chain.get_next_texture();
            {
                self.bottom_node.begin_render_pass(&frame, &mut encoder);

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Load,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: crate::utils::clear_color(),
                    }],
                    depth_stencil_attachment: Some(
                        crate::depth_stencil::create_attachment_descriptor(
                            &self.depth_texture_view,
                        ),
                    ),
                });
                rpass.set_pipeline(&self.pipeline);
                rpass.set_bind_group(0, &self.bind_group, &[]);
                rpass.set_index_buffer(&self.index_buf, 0);
                rpass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
                rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
            }

            self.app_view.device.get_queue().submit(&[encoder.finish()]);
        }
    }
}

impl PageTurning {
    fn touch_end(&mut self) {
        println!("touch_end--");
        let min_width = self.page_rect.width / 2.0;

        if self.roll_length < min_width {
            // 恢复原状
            println!("恢复原状--");
            self.interpolate_frame_data(self.touch_start, false);
        } else {
            println!("翻开--");
            // 从任意角度自然过渡到水平翻页状态
            let target_pos = Position::new(-self.page_rect.center_x() * 3.0, self.touch_start.y);
            self.interpolate_frame_data(target_pos, true);
        }
    }

    // 动画帧插值
    fn interpolate_frame_data(&mut self, target_pos: Position, is_open: bool) {
        let mut step_pixel = 30.0;
        let mut need_loop = true;
        while need_loop {
            let frame_count = (self.last_touch.distance(&target_pos) / step_pixel).ceil();
            if frame_count <= 1.1 {
                need_loop = false;
            } else {
                step_pixel -= 0.3;
                if step_pixel < 8.0 {
                    step_pixel = 8.0;
                }
            }

            let dx = (target_pos.x - self.last_touch.x) / frame_count;
            let dy = (target_pos.y - self.last_touch.y) / frame_count;
            let p = self.last_touch.offset(dx, dy);
            self.frame_data.push(p);
            self.last_touch = p;
        }
    }

    fn fresh_frame_data(&mut self) {
        if self.frame_data.is_empty() {
            self.is_animating = false;

            // 翻页动画结束
            if !self.is_touch_moving {
                println!("stopped===");
                self.if_need_stop = true;
                self.touch_start = Position::zero();
                self.turning_uniform.np = Position::zero().into();
                self.turning_uniform.n = Position::zero().into();
                // self.turning_uniform.alpha = 1.0;
                // crate::utils::update_uniform(device, self.turning_uniform, &self.turning_buf);
            }
        } else {
            let pos = self.frame_data.remove(0);
            self.step_frame_data(pos);
        }
    }

    fn step_frame_data(&mut self, position: Position) {
        let dx = self.touch_start.x - position.x;
        //无效的卷动:垂直 && dx < 0
        if dx <= 0.01 {
            return;
        }

        let dy = self.touch_start.y - position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let half_circle = std::f32::consts::PI * self.turning_uniform.radius;
        let pi_2 = std::f32::consts::PI / 2.0;

        let a = -dy.atan2(dx);
        let sin_a = a.sin();
        let cos_a = a.cos();
        // 最大可卷起距离
        let mut max_roll = 0.0;
        if a == 0.0 {
            max_roll = distance;
        } else if a < pi_2 && a > (-pi_2) {
            max_roll = (cos_a * (self.page_rect.width * 2.0)).abs();
        }
        // 实际的卷起距离
        self.roll_length = distance;
        if distance > half_circle {
            self.roll_length = (distance - half_circle) / 2.0 + half_circle;
        }
        if self.roll_length > max_roll {
            self.roll_length = max_roll;
        }
        self.turning_uniform.angle = a;
        self.turning_uniform.np = [
            self.page_rect.center_x() - (cos_a * self.roll_length).abs(),
            self.page_rect.center_y() * (if a > 0.0 { 1.0 } else { -1.0 })
                - sin_a * self.roll_length,
        ];
        self.turning_uniform.n = [cos_a, sin_a];
        crate::utils::update_uniform(
            &mut self.app_view.device,
            self.turning_uniform,
            &self.turning_buf,
        );
        println!(
            "max_roll: {}, a: {}, roll_lenght: {}, np: {:?}, px: {}, {}",
            max_roll, a, self.roll_length, self.turning_uniform.np, position.x, position.y
        );
    }
}
