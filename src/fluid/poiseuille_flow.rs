use crate::geometry::plane::Plane;
use crate::node::{F32TexNode, NoneNode};
use crate::texture;
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosTex};
use crate::SurfaceView;

use uni_view::{AppView, GPUContext};

use nalgebra_glm as glm;

use std::rc::Rc;

use super::ParticleNode;

// 泊萧叶流
pub struct PoiseuilleFlow {
    app_view: AppView,

    lattice_num_x: f32,
    lattice_num_y: f32,
    particle_num_x: f32,
    particle_num_y: f32,

    f0_view: wgpu::TextureView,
    f1_view: wgpu::TextureView,
    f2_view: wgpu::TextureView,
    uniform_buf: wgpu::Buffer,

    shader_collide: crate::shader::Shader,
    shader_propagate: crate::shader::Shader,
    shader_particle: crate::shader::Shader,

    bind_group0: wgpu::BindGroup,
    bind_group1: wgpu::BindGroup,

    propagate_pipeline: wgpu::ComputePipeline,
    collide_pipeline: wgpu::ComputePipeline,
    particle_pipeline: wgpu::ComputePipeline,

    final_node: ParticleNode,
    test_node: F32TexNode,

    swap: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FluidUniform {
    e: [[f32; 2]; 9],
    // lattice 在正规化坐标空间的大小
    lattice_size: [f32; 2],
    // 格子数
    lattice_num: [f32; 2],
    weight: [f32; 9],
    swap: i32,
}

fn get_fluid_uniform(lattice_num_x: f32, lattice_num_y: f32, swap: i32) -> FluidUniform {
    let w0 = 4.0 / 9.0;
    let w1 = 1.0 / 9.0;
    let w2 = 1.0 / 36.0;
    // cell structure (subcell numbers):
    // 6 2 5
    // 3 0 1
    // 7 4 8
    // 按钮 imageLoad 坐标取值的特点来指定方向坐标
    let e: [[f32; 2]; 9] = [
        [0.0, 0.0],
        [1.0, 0.0],
        [0.0, -1.0],
        [-1.0, 0.0],
        [0.0, 1.0],
        [1.0, -1.0],
        [-1.0, -1.0],
        [-1.0, 1.0],
        [1.0, 1.0],
    ];
    let weight: [f32; 9] = [w0, w1, w1, w1, w1, w2, w2, w2, w2];
    FluidUniform {
        e,
        lattice_size: [2.0 / lattice_num_x, 2.0 / lattice_num_y],
        lattice_num: [lattice_num_x, lattice_num_y],
        weight,
        swap,
    }
}

impl PoiseuilleFlow {
    pub fn new(app_view: AppView) -> Self {
        use std::mem;
        let mut app_view = app_view;
        let lattice_num_x = 100.0;
        let lattice_num_y = 100.0;
        let particle_num_x = 100.0;
        let particle_num_y = 100.0;

        let swap = 0_i32;

        let mut encoder =
            app_view.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        // Create the texture
        let f0_view = texture::empty_f32_view(
            &mut app_view.device,
            lattice_num_x as u32,
            lattice_num_y as u32,
        );
        let f1_view = texture::empty_f32_view(
            &mut app_view.device,
            lattice_num_x as u32,
            lattice_num_y as u32,
        );
        let f2_view = texture::empty_f32_view(
            &mut app_view.device,
            lattice_num_x as u32,
            lattice_num_y as u32,
        );
        let f11_view = texture::empty_f32_view(
            &mut app_view.device,
            lattice_num_x as u32,
            lattice_num_y as u32,
        );
        let f21_view = texture::empty_f32_view(
            &mut app_view.device,
            lattice_num_x as u32,
            lattice_num_y as u32,
        );
        let macro_view = texture::empty_f32_view(
            &mut app_view.device,
            lattice_num_x as u32,
            lattice_num_y as u32,
        );

        let uniform_buf = crate::utils::create_uniform_buffer(
            &mut app_view.device,
            get_fluid_uniform(lattice_num_x, lattice_num_y, swap),
        );

        // Create pipeline layout
        let bind_group_layout =
            app_view.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    wgpu::BindGroupLayoutBinding {
                        binding: 2,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture,
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 3,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture,
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 4,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture,
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 5,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture,
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 6,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture,
                    },
                ],
            });

        let bind_group0 = app_view.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0..(std::mem::size_of::<FluidUniform>() as wgpu::BufferAddress),
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&f0_view),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&f1_view),
                },
                wgpu::Binding {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&f2_view),
                },
                wgpu::Binding {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&f11_view),
                },
                wgpu::Binding {
                    binding: 5,
                    resource: wgpu::BindingResource::TextureView(&f21_view),
                },
                wgpu::Binding {
                    binding: 6,
                    resource: wgpu::BindingResource::TextureView(&macro_view),
                },
            ],
        });
        let pipeline_layout =
            app_view.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            });
        // Create the render pipeline
        let shader_init =
            crate::shader::Shader::new_by_compute("fluid/poiseuille_init", &mut app_view.device);
        let shader_propagate = crate::shader::Shader::new_by_compute(
            "fluid/poiseuille_propagate",
            &mut app_view.device,
        );

        let shader_collide =
            crate::shader::Shader::new_by_compute("fluid/poiseuille_collide", &mut app_view.device);
        let shader_particle = crate::shader::Shader::new_by_compute(
            "fluid/poiseuille_particle",
            &mut app_view.device,
        );

        let init_pipeline =
            app_view.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                layout: &pipeline_layout,
                compute_stage: shader_init.cs_stage(),
            });
        let propagate_pipeline =
            app_view.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                layout: &pipeline_layout,
                compute_stage: shader_propagate.cs_stage(),
            });
        let collide_pipeline =
            app_view.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                layout: &pipeline_layout,
                compute_stage: shader_collide.cs_stage(),
            });

        let mvp = crate::matrix_helper::default_mvp(&app_view.sc_desc);
        // let mvp = crate::matrix_helper::ortho_default_mvp();

        let final_node = ParticleNode::new(
            &app_view.sc_desc,
            &mut app_view.device,
            MVPUniform { mvp_matrix: mvp },
            particle_num_x as u32,
            particle_num_y as u32,
        );

        let bind_group_layout1 =
            app_view.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    wgpu::BindGroupLayoutBinding {
                        binding: 2,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture,
                    },
                ],
            });

        let uniform_buf2 = crate::utils::create_uniform_buffer(
            &mut app_view.device,
            get_fluid_uniform(lattice_num_x, lattice_num_y, swap),
        );
        let bind_group1 = app_view.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout1,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf2,
                        range: 0..(std::mem::size_of::<FluidUniform>() as wgpu::BufferAddress),
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&final_node.particle_position_tv),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&macro_view),
                },
            ],
        });
        let pipeline_layout1 =
            app_view.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout1],
            });
        let particle_pipeline =
            app_view.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                layout: &pipeline_layout1,
                compute_stage: shader_particle.cs_stage(),
            });

        {
            let mut cpass = encoder.begin_compute_pass();
            cpass.set_pipeline(&init_pipeline);
            cpass.set_bind_group(0, &bind_group0, &[]);
            cpass.dispatch(lattice_num_x as u32, lattice_num_y as u32, 1);
        }
        app_view.device.get_queue().submit(&[encoder.finish()]);

        let test_node = F32TexNode::new(
            &app_view.sc_desc,
            &mut app_view.device,
            &macro_view,
            MVPUniform { mvp_matrix: mvp },
        );

        PoiseuilleFlow {
            app_view,
            lattice_num_x,
            lattice_num_y,
            particle_num_x,
            particle_num_y,
            f0_view,
            f1_view,
            f2_view,
            uniform_buf,
            shader_collide,
            shader_particle,
            shader_propagate,
            bind_group0,
            bind_group1,
            propagate_pipeline,
            collide_pipeline,
            particle_pipeline,
            final_node,
            test_node,
            swap,
        }
    }

    fn fluid_compute(&self, encoder: &mut wgpu::CommandEncoder) {
        {
            let mut cpass = encoder.begin_compute_pass();
            cpass.set_pipeline(&self.propagate_pipeline);
            cpass.set_bind_group(0, &self.bind_group0, &[]);
            cpass.dispatch(self.lattice_num_x as u32, self.lattice_num_y as u32, 1);
        }
        {
            let mut cpass = encoder.begin_compute_pass();
            cpass.set_pipeline(&self.collide_pipeline);
            cpass.set_bind_group(0, &self.bind_group0, &[]);
            cpass.dispatch(self.lattice_num_x as u32, self.lattice_num_y as u32, 1);
        }
        {
            // 按粒子数执行
            let mut cpass = encoder.begin_compute_pass();
            cpass.set_pipeline(&self.particle_pipeline);
            cpass.set_bind_group(0, &self.bind_group1, &[]);
            cpass.dispatch(self.particle_num_x as u32, self.particle_num_y as u32, 1);
        }
    }
}

impl SurfaceView for PoiseuilleFlow {
    fn update(&mut self, _event: wgpu::winit::WindowEvent) {
        //empty
    }

    fn touch_moved(&mut self, _position: crate::math::Position) {}

    fn resize(&mut self) {
        self.app_view.update_swap_chain();
    }

    fn enter_frame(&mut self) {
        let mut encoder = self
            .app_view
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        self.fluid_compute(&mut encoder);

        let frame = self.app_view.swap_chain.get_next_texture();

        {
            // self.final_node.begin_render_pass(&frame.view, &mut encoder);
            self.test_node.begin_render_pass(&frame, &mut encoder);
        }

        self.app_view.device.get_queue().submit(&[encoder.finish()]);

        // if self.swap == 0 {
        //     self.swap = 1;
        // } else {
        //     self.swap = 0;
        // }
        // let uniform = get_fluid_uniform(self.lattice_num_x, self.lattice_num_y, self.swap);
        // crate::utils::update_uniform(&mut self.app_view.device, uniform, &self.uniform_buf);
    }
}
