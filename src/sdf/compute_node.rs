#[derive(Copy, Clone)]
#[repr(C)]
pub struct PicInfoUniform {
    info: [i32; 4],
}

pub struct SDFComputeNode {
    extent: wgpu::Extent3d,
    bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    compute_pipeline: wgpu::ComputePipeline,
}

impl SDFComputeNode {
    pub fn new(
        device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder,
        src_view: &wgpu::TextureView, extent: wgpu::Extent3d,
    ) -> Self {
        let img_size = (extent.width * extent.height) as u64;
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 1,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 2,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageBuffer { dynamic: false, readonly: false },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 3,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageBuffer { dynamic: false, readonly: false },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 4,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageBuffer { dynamic: false, readonly: false },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 5,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageBuffer { dynamic: false, readonly: false },
                },
            ],
        });

        let uniform_size = std::mem::size_of::<PicInfoUniform>() as wgpu::BufferAddress;
        let uniform_buf = crate::utils::create_uniform_buffer2(
            device,
            encoder,
            PicInfoUniform { info: [0, 0, 0, 0] },
            uniform_size,
        );

        let sdf_range = (img_size * 4) as wgpu::BufferAddress;
        let sdf_front = device.create_buffer(&wgpu::BufferDescriptor {
            size: sdf_range,
            usage: wgpu::BufferUsage::STORAGE,
        });
        let sdf_background = device.create_buffer(&wgpu::BufferDescriptor {
            size: sdf_range,
            usage: wgpu::BufferUsage::STORAGE,
        });

        let v_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: sdf_range,
            usage: wgpu::BufferUsage::STORAGE,
        });
        let z_range = ((extent.width + 1) * (extent.height + 1) * 4) as wgpu::BufferAddress;
        let z_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: z_range,
            usage: wgpu::BufferUsage::STORAGE,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0..uniform_size,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(src_view),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &sdf_front,
                        range: 0..sdf_range,
                    },
                },
                wgpu::Binding {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &sdf_background,
                        range: 0..sdf_range,
                    },
                },
                wgpu::Binding {
                    binding: 4,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &v_buffer,
                        range: 0..sdf_range,
                    },
                },
                wgpu::Binding {
                    binding: 5,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &z_buffer,
                        range: 0..z_range,
                    },
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let shader_compute = crate::shader::Shader::new_by_compute("sdf/sdf", device);
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: &pipeline_layout,
            compute_stage: shader_compute.cs_stage(),
        });
        SDFComputeNode { extent, uniform_buf, bind_group, compute_pipeline }
    }

    pub fn compute(&mut self, device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        // init distance fields
        self.step_uniform(2, 0, self.extent.width, self.extent.height, device, encoder);
        // crate::utils::update_buffer_use_encoder(encoder, device, self.step_uniform(2, 0), &self.uniform_buf);

        // step front y
        self.step_uniform(0, 0, self.extent.width, 1, device, encoder);

        // crate::utils::update_buffer_use_encoder(encoder, device, self.step_uniform(0, 0), &self.uniform_buf);
        // cpass.dispatch(self.extent.width, 1, 1);

        // step front x
        self.step_uniform(1, 0, 1, self.extent.height, device, encoder);

        // // crate::utils::update_buffer_use_encoder(encoder, device, self.step_uniform(1, 0), &self.uniform_buf);
        // // cpass.dispatch(1, self.extent.height, 1);

        // step background y
        self.step_uniform(0, 1, self.extent.width, 1, device, encoder);

        // // crate::utils::update_buffer_use_encoder(encoder, device, self.step_uniform(0, 1), &self.uniform_buf);
        // // cpass.dispatch(self.extent.width, 1, 1);
        // step background x
        self.step_uniform(1, 1, 1, self.extent.height, device, encoder);

        // // crate::utils::update_buffer_use_encoder(encoder, device, self.step_uniform(1, 1), &self.uniform_buf);
        // // cpass.dispatch(1, self.extent.height, 1);

        // final output
        self.step_uniform(3, 0, self.extent.width as u32, self.extent.height, device, encoder);
        // crate::utils::update_buffer_use_encoder(encoder, device, self.step_uniform(3, 0), &self.uniform_buf);
        // cpass.dispatch(self.extent.width, self.extent.height, 1);
    }

    fn step_uniform(
        &self, iter: i32, is_bg: i32, dispatch_x: u32, dispatch_y: u32, device: &mut wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let u = PicInfoUniform {
            info: [self.extent.width as i32, self.extent.height as i32, iter, is_bg],
        };
        crate::utils::update_buffer_use_encoder(encoder, device, u, &self.uniform_buf);
        

        let mut cpass = encoder.begin_compute_pass();
        cpass.set_pipeline(&self.compute_pipeline);
        cpass.set_bind_group(0, &self.bind_group, &[]);
        cpass.dispatch(dispatch_x, dispatch_y, 1);
    }
}
