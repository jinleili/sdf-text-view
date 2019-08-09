use crate::node::{F32BufferNode, NoneNode};
use crate::texture;
use crate::utils::MVPUniform;
use crate::SurfaceView;

use uni_view::{AppView, GPUContext};
use super::{ParticleNode, DensityNode};

// 泊萧叶流
pub struct PoiseuilleFlow {
    app_view: AppView,

    lattice_num_x: i32,
    lattice_num_y: i32,
    particle_num_x: f32,
    particle_num_y: f32,

    uniform_buf: wgpu::Buffer,
    lattice0_buffer: wgpu::Buffer,
    lattice1_buffer: wgpu::Buffer,
    fluid_buffer: wgpu::Buffer,

    staging_buffer: wgpu::Buffer,

    staging_lattice0: wgpu::Buffer,
    staging_lattice1: wgpu::Buffer,

    shader_collide: crate::shader::Shader,
    shader_propagate: crate::shader::Shader,

    bind_group0: wgpu::BindGroup,
    bind_group1: wgpu::BindGroup,

    propagate_pipeline: wgpu::ComputePipeline,
    collide_pipeline: wgpu::ComputePipeline,
    particle_pipeline: wgpu::ComputePipeline,

    final_node: ParticleNode,
    test_node: F32BufferNode,
    density_node: DensityNode,

    swap: i32,
    isCopingData: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FluidUniform {
    // 由于 OpenGL spec 定义的对齐方式，非 标量 或 vec2, 都是按 16 字节对齐
    // https://github.com/gfx-rs/wgpu-rs/issues/36
    e_and_w: [[f32; 4]; 9],
    // lattice 在正规化坐标空间的大小
    lattice_size: [f32; 2],
    // 格子数
    lattice_num: [i32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FluidCell {
    color: [f32; 4],
}

fn init_data(nx: u32, ny: u32) -> (Vec<f32>, Vec<FluidCell>) {
    let w0 = 4.0 / 9.0;
    let w1 = 1.0 / 9.0;
    let w2 = 1.0 / 36.0;
    let mut lattice: Vec<f32> = vec![];
    let mut fluid: Vec<FluidCell> = vec![];
    let weight = vec![w0, w1, w1, w1, w1, w2, w2, w2, w2];
    for j in 0..ny {
        for i in 0..nx {
            for k in 0..9 {
                lattice.push(weight[k]);
            }
            fluid.push(FluidCell { color: [0.0, 0.0, 0.0, setup_open_geometry(i, j, nx, ny) as f32] });
        }
    }
    
    (lattice, fluid)
}

fn setup_open_geometry(x: u32, y: u32, nx: u32, ny: u32) -> u32 {
    // 不同的边用 10 的倍数来表示？
    if x == 0 || y == 0 || x == nx-1 || y == ny-1 {
		return 0; // disable end of world
	}
	if (x == 1 || x == nx-2) && (y > 0 && y < ny-1) {
		if x == 1 && y > 1 && y < ny-2 {
			return 5; // inflow
		}
		if x == nx-2 && y > 1 && y < ny-2 {
			return 6; // outflow
		}
		return 2; // bounce back outer walls
	}
	if (y == 1 || y == ny-2) && (x > 0 && x < nx-1) {
		return 2; // bounce back outer walls
	}
	return 1; // everything else shall be bulk fluid
}

fn get_fluid_uniform(lattice_num_x: i32, lattice_num_y: i32) -> FluidUniform {
    let w0 = 4.0 / 9.0;
    let w1 = 1.0 / 9.0;
    let w2 = 1.0 / 36.0;
    // cell structure (subcell numbers):
    // 6 2 5
    // 3 0 1
    // 7 4 8
    // 按钮 imageLoad 坐标取值的特点来指定方向坐标
    let e_and_w: [[f32; 4]; 9] = [
        [0.0, 0.0, w0, 0.0],
        [1.0, 0.0, w1, 0.0],
        [0.0, -1.0, w1, 0.0],
        [-1.0, 0.0, w1, 0.0],
        [0.0, 1.0, w1, 0.0],
        [1.0, -1.0, w2, 0.0],
        [-1.0, -1.0, w2, 0.0],
        [-1.0, 1.0, w2, 0.0],
        [1.0, 1.0, w2, 0.0],
    ];

    FluidUniform {
        e_and_w,
        lattice_size: [2.0 / lattice_num_x as f32, 2.0 / lattice_num_y as f32],
        lattice_num: [lattice_num_x, lattice_num_y],
    }
}

impl PoiseuilleFlow {
    pub fn new(app_view: AppView) -> Self {
        use std::mem;
        let mut app_view = app_view;

        let lattice_num = 10;

        let lattice_num_x = lattice_num;
        let lattice_num_y = lattice_num;
        let particle_num_x = lattice_num as f32 * 4.0;
        let particle_num_y = lattice_num as f32 * 4.0;

        let swap = 0_i32;

        let mut encoder =
            app_view.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        // 格子 buffer 所占字节数
        let buffer_range = (lattice_num_x * lattice_num_y * 9 * 4) as wgpu::BufferAddress;
        // 输出的流体参数 buffer 的字节数
        let fluid_buf_range = (lattice_num_x * lattice_num_y) as wgpu::BufferAddress
            * (std::mem::size_of::<FluidCell>() as wgpu::BufferAddress);

        let (lattice_data, fluid_data) = init_data(lattice_num_x as u32, lattice_num_y as u32);
        let (lattice0_buffer, staging_lattice0) = crate::utils::create_storage_buffer(
            &mut app_view.device,
            &mut encoder,
            &lattice_data,
            buffer_range,
        );
        let (lattice1_buffer, staging_lattice1) = crate::utils::create_storage_buffer(
            &mut app_view.device,
            &mut encoder,
            &lattice_data,
            buffer_range,
        );
        let (fluid_buffer, staging_buffer) = crate::utils::create_storage_buffer(
            &mut app_view.device,
            &mut encoder,
            &fluid_data,
            fluid_buf_range,
        );

        let uniform_size = std::mem::size_of::<FluidUniform>() as wgpu::BufferAddress;
        let uniform_buf = crate::utils::create_uniform_buffer2(
            &mut app_view.device,
            &mut encoder,
            get_fluid_uniform(lattice_num_x, lattice_num_y),
            uniform_size,
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
                        ty: wgpu::BindingType::StorageBuffer,
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 2,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageBuffer,
                    },
                    wgpu::BindGroupLayoutBinding {
                        binding: 3,
                        visibility: wgpu::ShaderStage::COMPUTE,
                        ty: wgpu::BindingType::StorageBuffer,
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
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &lattice0_buffer,
                        range: 0..buffer_range,
                    },
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &lattice1_buffer,
                        range: 0..buffer_range,
                    },
                },
                wgpu::Binding {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &fluid_buffer,
                        range: 0..fluid_buf_range,
                    },
                },
            ],
        });
        let pipeline_layout =
            app_view.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            });
        // Create the render pipeline
        let shader_propagate = crate::shader::Shader::new_by_compute(
            "fluid2/poiseuille_propagate",
            &mut app_view.device,
        );
        let propagate_pipeline =
            app_view.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                layout: &pipeline_layout,
                compute_stage: shader_propagate.cs_stage(),
            });

        let shader_collide = crate::shader::Shader::new_by_compute(
            "fluid2/poiseuille_collide",
            &mut app_view.device,
        );
        let collide_pipeline =
            app_view.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                layout: &pipeline_layout,
                compute_stage: shader_collide.cs_stage(),
            });

        let mvp = crate::matrix_helper::default_mvp(&app_view.sc_desc);
        // let mvp = crate::matrix_helper::ortho_default_mvp();

        // 目前的实现，粒子数需要与格子数一致
        let final_node = ParticleNode::new(
            &app_view.sc_desc,
            &mut app_view.device,
            MVPUniform { mvp_matrix: mvp },
            particle_num_x as u32,
            particle_num_y as u32,
            &fluid_buffer,
        );

        let density_node = DensityNode::new(
            &app_view.sc_desc,
            &mut app_view.device,
            MVPUniform { mvp_matrix: mvp },
            particle_num_x as u32,
            particle_num_y as u32,
            &fluid_buffer,
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
                        ty: wgpu::BindingType::StorageBuffer,
                    },
                ],
            });

        let bind_group1 = app_view.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout1,
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
                    resource: wgpu::BindingResource::TextureView(&final_node.particle_position_tv),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &fluid_buffer,
                        range: 0..fluid_buf_range,
                    },
                }
            ],
        });
        let pipeline_layout1 =
            app_view.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout1],
            });

        let shader_particle = crate::shader::Shader::new_by_compute(
            "fluid2/poiseuille_particle",
            &mut app_view.device,
        );
        let particle_pipeline =
            app_view.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                layout: &pipeline_layout1,
                compute_stage: shader_particle.cs_stage(),
            });

        app_view.device.get_queue().submit(&[encoder.finish()]);

        let test_node = F32BufferNode::new(
            &app_view.sc_desc,
            &mut app_view.device,
            &fluid_buffer,
            fluid_buf_range,
            MVPUniform { mvp_matrix: mvp },
        );

        PoiseuilleFlow {
            app_view,
            lattice_num_x,
            lattice_num_y,
            particle_num_x,
            particle_num_y,

            uniform_buf,
            lattice0_buffer,
            lattice1_buffer,
            fluid_buffer,
            staging_buffer,
            staging_lattice0,
            staging_lattice1,

            shader_collide,
            shader_propagate,

            bind_group0,
            bind_group1,
            propagate_pipeline,
            collide_pipeline,
            particle_pipeline,
            final_node,
            test_node,
            density_node,
            swap,
            isCopingData: false,
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

        let fluid_buf_range = (self.lattice_num_x * self.lattice_num_y) as wgpu::BufferAddress
            * (std::mem::size_of::<FluidCell>() as wgpu::BufferAddress);
        let lattice_range =
            (self.lattice_num_x * self.lattice_num_y * 9 * 4) as wgpu::BufferAddress;

        if self.isCopingData == false {
            self.swap = 0;
            self.isCopingData = true;

            self.fluid_compute(&mut encoder);

            encoder.copy_buffer_to_buffer(
                &self.fluid_buffer,
                0,
                &self.staging_buffer,
                0,
                fluid_buf_range,
            );
            // encoder.copy_buffer_to_buffer(
            //     &self.lattice0_buffer,
            //     0,
            //     &self.staging_lattice0,
            //     0,
            //     lattice_range,
            // );
            // encoder.copy_buffer_to_buffer(
            //     &self.lattice1_buffer,
            //     0,
            //     &self.staging_lattice1,
            //     0,
            //     lattice_range,
            // );
        }
        let frame = self.app_view.swap_chain.get_next_texture();
        {
            // self.final_node.begin_render_pass(&frame.view, &mut encoder);
            self.density_node.begin_render_pass(&frame.view, &mut encoder);
            // self.test_node.begin_render_pass(&frame, &mut encoder);
        }
        self.app_view.device.get_queue().submit(&[encoder.finish()]);

        if self.isCopingData {
            if self.swap == 0 {
                self.staging_buffer.map_read_async(
                    0,
                    fluid_buf_range,
                    |result: wgpu::BufferMapAsyncResult<&[f32]>| {
                        println!("{:?}", result.unwrap().data);
                    },
                );
                // self.staging_lattice0.map_read_async(
                //     0,
                //     lattice_range,
                //     |result: wgpu::BufferMapAsyncResult<&[f32]>| {
                //         println!("staging_lattice0:");
                //         println!("{:?}", result.unwrap().data);
                //     },
                // );
                // self.staging_lattice1.map_read_async(
                //     0,
                //     lattice_range,
                //     |result: wgpu::BufferMapAsyncResult<&[f32]>| {
                //         println!("staging_lattice1:");
                //         println!("{:?}", result.unwrap().data);
                //     },
                // );
            } else if self.swap == 50 {
                self.isCopingData = false;
            }
            self.swap += 1;
        }
    }
}
