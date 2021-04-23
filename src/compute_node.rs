use crate::PicInfoUniform;
use libc::NOFLSH;
use wgpu::util::DeviceExt;
use zerocopy::AsBytes;

pub struct SDFComputeNode {
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
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    count: None,
                    binding: 0,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(0),
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    count: None,
                    binding: 1,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        view_dimension: wgpu::TextureViewDimension::D2,
                        access: wgpu::StorageTextureAccess::ReadWrite,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    count: None,
                    binding: 2,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(0),
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    count: None,
                    binding: 3,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(0),
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    count: None,
                    binding: 4,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(0),
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    count: None,
                    binding: 5,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(0),
                    },
                },
            ],
        });

        let offset_stride = std::mem::size_of::<PicInfoUniform>() as wgpu::BufferAddress;
        let uniform_size = offset_stride * 6;
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &[
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
            ]
            .as_bytes(),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let sdf_range = (img_size * 4) as wgpu::BufferAddress;
        let sdf_front = device.create_buffer(&wgpu::BufferDescriptor {
            size: sdf_range,
            usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_SRC,
            label: None,
            mapped_at_creation: false,
        });
        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: sdf_range,
            usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
            label: None,
            mapped_at_creation: false,
        });
        let sdf_background = device.create_buffer(&wgpu::BufferDescriptor {
            size: sdf_range,
            usage: wgpu::BufferUsage::STORAGE,
            label: None,
            mapped_at_creation: false,
        });

        let v_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: sdf_range,
            usage: wgpu::BufferUsage::STORAGE,
            label: None,
            mapped_at_creation: false,
        });
        let z_range = ((extent.width + 1) * (extent.height + 1) * 4) as wgpu::BufferAddress;
        let z_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: z_range,
            usage: wgpu::BufferUsage::STORAGE,
            label: None,
            mapped_at_creation: false,
        });
        let bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            label: None,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform_buffer.as_entire_binding() },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(src_view),
                },
                wgpu::BindGroupEntry { binding: 2, resource: sdf_front.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: sdf_background.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 4, resource: v_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 5, resource: z_buffer.as_entire_binding() },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            push_constant_ranges: &[],
            bind_group_layouts: &[&bind_group_layout],
        });

        let shader_xy = idroid::shader::Shader::new_by_compute("sdf/sdf", device);
        let xy_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: Some(&pipeline_layout),
            module: &shader_xy.vs_module,
            entry_point: "main",
            label: None,
        });
        let shader_x = idroid::shader::Shader::new_by_compute("sdf/sdf_x", device);
        let x_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: Some(&pipeline_layout),
            module: &shader_x.vs_module,
            entry_point: "main",
            label: None,
        });
        let shader_y = idroid::shader::Shader::new_by_compute("sdf/sdf_y", device);
        let y_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: Some(&pipeline_layout),
            module: &shader_y.vs_module,
            entry_point: "main",
            label: None,
        });

        let threadgroup_count = ((extent.width + 15) / 16, (extent.height + 15) / 16);

        SDFComputeNode {
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
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&self.xy_pipeline);
        cpass.set_bind_group(0, &self.bind_group, &[0]);
        cpass.dispatch(self.threadgroup_count.0, self.threadgroup_count.1, 1);

        cpass.set_pipeline(&self.x_pipeline);
        // step background y
        cpass.set_bind_group(0, &self.bind_group, &[self.offset_stride as wgpu::DynamicOffset * 3]);
        cpass.dispatch(self.threadgroup_count.0, 1, 1);
        // step front y
        cpass.set_bind_group(0, &self.bind_group, &[self.offset_stride as wgpu::DynamicOffset]);
        cpass.dispatch(self.threadgroup_count.0, 1, 1);

        cpass.set_pipeline(&self.y_pipeline);
        // step background x
        cpass.set_bind_group(0, &self.bind_group, &[self.offset_stride as wgpu::DynamicOffset * 4]);
        cpass.dispatch(1, self.threadgroup_count.1, 1);
        // step front x
        cpass.set_bind_group(0, &self.bind_group, &[self.offset_stride as wgpu::DynamicOffset * 2]);
        cpass.dispatch(1, self.threadgroup_count.1, 1);

        // final output
        cpass.set_pipeline(&self.xy_pipeline);
        cpass.set_bind_group(0, &self.bind_group, &[self.offset_stride as wgpu::DynamicOffset * 5]);
        cpass.dispatch(self.threadgroup_count.0, self.threadgroup_count.1, 1);
    }
}
