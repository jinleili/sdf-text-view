#[derive(Copy, Clone)]
#[repr(C)]
pub struct PicInfoUniform {
    info: [i32; 4],
}

pub struct SDFComputeNode {
    extent: wgpu::Extent3d,
    bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    init_pipeline: wgpu::ComputePipeline,
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
                wgpu::BindGroupLayoutBinding {
                    binding: 6,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageBuffer { dynamic: false, readonly: false },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 7,
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
        let sdf_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: sdf_range,
            usage: wgpu::BufferUsage::STORAGE
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
        });

        let max_length = if extent.width > extent.height { extent.width } else { extent.height };
        let f_range = (max_length * 4) as wgpu::BufferAddress;
        let f_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: f_range,
            usage: wgpu::BufferUsage::STORAGE
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
        });

        // temp distance, default value is set to zero
        let d_data = vec![0.0_f32; max_length as usize];
        let (d_buffer, _) = crate::utils::create_storage_buffer(device, encoder, &d_data, f_range);

        let v_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: f_range,
            usage: wgpu::BufferUsage::STORAGE
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
        });
        let z_range = ((max_length + 1) * 4) as wgpu::BufferAddress;
        let z_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: z_range,
            usage: wgpu::BufferUsage::STORAGE
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
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
                        buffer: &sdf_buffer,
                        range: 0..sdf_range,
                    },
                },
                wgpu::Binding {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &sdf_buffer,
                        range: 0..sdf_range,
                    },
                },
                wgpu::Binding {
                    binding: 4,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &f_buffer,
                        range: 0..f_range,
                    },
                },
                wgpu::Binding {
                    binding: 5,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &d_buffer,
                        range: 0..f_range,
                    },
                },
                wgpu::Binding {
                    binding: 6,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &v_buffer,
                        range: 0..f_range,
                    },
                },
                wgpu::Binding {
                    binding: 7,
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

        let shader_init = crate::shader::Shader::new_by_compute("sdf/sdf_init", device);
        let init_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: &pipeline_layout,
            compute_stage: shader_init.cs_stage(),
        });

        let shader_compute = crate::shader::Shader::new_by_compute("sdf/sdf", device);
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: &pipeline_layout,
            compute_stage: shader_compute.cs_stage(),
        });
        SDFComputeNode { extent, uniform_buf, bind_group, init_pipeline, compute_pipeline }
    }

    pub fn compute(&self, device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        {
            let mut cpass = encoder.begin_compute_pass();
            cpass.set_pipeline(&self.init_pipeline);
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.dispatch(self.extent.width, self.extent.height, 1);
        }
        // step front y
        self.step_cul(0, 0, self.extent.width, 1, device, encoder);
        // step front x
        self.step_cul(1, 0, 1, self.extent.height, device, encoder);
        // step background y
        self.step_cul(0, 1, self.extent.width, 1, device, encoder);
        // step background x
        self.step_cul(1, 1, 1, self.extent.height, device, encoder);
        // final output
        self.step_cul(2, 0, self.extent.width, self.extent.height, device, encoder);
    }

    fn step_cul(
        &self, iter: i32, is_bg: i32, dispatch_x: u32, dispatch_y: u32, device: &mut wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let u = PicInfoUniform {
            info: [self.extent.width as i32, self.extent.height as i32, iter, is_bg],
        };
        crate::utils::update_uniform(device, u, &self.uniform_buf);
        let mut cpass = encoder.begin_compute_pass();
        cpass.set_pipeline(&self.compute_pipeline);
        cpass.set_bind_group(0, &self.bind_group, &[]);
        cpass.dispatch(dispatch_x, dispatch_y, 1);
    }
}
