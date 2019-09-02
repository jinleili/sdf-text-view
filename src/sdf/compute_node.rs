#[derive(Copy, Clone)]
#[repr(C)]
pub struct PicInfoUniform {
    info: [i32; 4],
    // only for requested 256 alignment: (256 - 16) / 4 = 60
    any: [i32; 60],
}

pub struct SDFComputeNode {
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    xy_pipeline: wgpu::ComputePipeline,
    x_pipeline: wgpu::ComputePipeline,
    y_pipeline: wgpu::ComputePipeline,
    offset_stride: wgpu::BufferAddress,
    threadgroup_count: (u32, u32),
    pub sdf_buffer: wgpu::Buffer,
    pub staging_buffer: wgpu::Buffer,
}

impl SDFComputeNode {
    pub fn new(
        device: &mut wgpu::Device, _encoder: &mut wgpu::CommandEncoder,
        src_view: &wgpu::TextureView, extent: wgpu::Extent3d,
    ) -> Self {
        let img_size = (extent.width * extent.height) as u64;
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: true },
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

        let offset_stride = std::mem::size_of::<PicInfoUniform>() as wgpu::BufferAddress;
        let uniform_size = offset_stride * 6;
        let uniform_buffer = device
            .create_buffer_mapped(6, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
            .fill_from_slice(&[
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 2, 0],
                    any: [0; 60],
                },
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 0, 0],
                    any: [0; 60],
                },
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 1, 0],
                    any: [0; 60],
                },
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 0, 1],
                    any: [0; 60],
                },
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 1, 1],
                    any: [0; 60],
                },
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 3, 0],
                    any: [0; 60],
                },
            ]);

        let sdf_range = (img_size * 4) as wgpu::BufferAddress;
        let sdf_front = device.create_buffer(&wgpu::BufferDescriptor {
            size: sdf_range,
            usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_SRC,
        });
        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: sdf_range,
            usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
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
        let bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buffer,
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

        let shader_xy = crate::shader::Shader::new_by_compute("sdf/sdf", device);
        let xy_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: &pipeline_layout,
            compute_stage: shader_xy.cs_stage(),
        });
        let shader_x = crate::shader::Shader::new_by_compute("sdf/sdf_x", device);
        let x_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: &pipeline_layout,
            compute_stage: shader_x.cs_stage(),
        });
        let shader_y = crate::shader::Shader::new_by_compute("sdf/sdf_y", device);
        let y_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: &pipeline_layout,
            compute_stage: shader_y.cs_stage(),
        });

        let threadgroup_count = ((extent.width + 15) / 16, (extent.height + 15) / 16);

        SDFComputeNode {
            uniform_buffer,
            bind_group,
            xy_pipeline,
            x_pipeline,
            y_pipeline,
            offset_stride,
            threadgroup_count,
            staging_buffer,
            sdf_buffer: sdf_front,
        }
    }

    pub fn compute(&mut self, _device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass();
        cpass.set_pipeline(&self.xy_pipeline);
        cpass.set_bind_group(0, &self.bind_group, &[0]);
        cpass.dispatch(self.threadgroup_count.0, self.threadgroup_count.1, 1);

        cpass.set_pipeline(&self.x_pipeline);
        // step background y
        cpass.set_bind_group(0, &self.bind_group, &[self.offset_stride * 3]);
        cpass.dispatch(self.threadgroup_count.0, 1, 1);
        // step front y
        cpass.set_bind_group(0, &self.bind_group, &[self.offset_stride]);
        cpass.dispatch(self.threadgroup_count.0, 1, 1);

        cpass.set_pipeline(&self.y_pipeline);
        // step background x
        cpass.set_bind_group(0, &self.bind_group, &[self.offset_stride * 4]);
        cpass.dispatch(1, self.threadgroup_count.1, 1);
        // step front x
        cpass.set_bind_group(0, &self.bind_group, &[self.offset_stride * 2]);
        cpass.dispatch(1, self.threadgroup_count.1, 1);

        // final output
        cpass.set_pipeline(&self.xy_pipeline);
        cpass.set_bind_group(0, &self.bind_group, &[self.offset_stride * 5]);
        cpass.dispatch(self.threadgroup_count.0, self.threadgroup_count.1, 1);
    }
}
