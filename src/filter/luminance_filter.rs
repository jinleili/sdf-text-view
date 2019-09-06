use crate::PicInfoUniform;
use idroid::texture;

#[allow(dead_code)]
pub struct LuminanceFilter {
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::ComputePipeline,
    offset_stride: wgpu::BufferAddress,
    threadgroup_count: (u32, u32),
    pub output_view: wgpu::TextureView,
}

#[allow(dead_code)]
impl LuminanceFilter {
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
                    ty: wgpu::BindingType::StorageTexture {
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                },
            ],
        });

        let offset_stride = std::mem::size_of::<PicInfoUniform>() as wgpu::BufferAddress;
        let uniform_size = offset_stride * 2;
        let uniform_buffer = device
            .create_buffer_mapped(2, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
            .fill_from_slice(&[
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 0, 0],
                    any: [0; 60],
                },
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 1, 0],
                    any: [0; 60],
                },
            ]);

        let output_view = texture::empty(device, wgpu::TextureFormat::R8Unorm, extent);
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
                    resource: wgpu::BindingResource::TextureView(&output_view),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let shader = idroid::shader::Shader::new_by_compute(
            "filter/luminance",
            device,
            env!("CARGO_MANIFEST_DIR"),
        );
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: &pipeline_layout,
            compute_stage: shader.cs_stage(),
        });

        let threadgroup_count = ((extent.width + 15) / 16, (extent.height + 15) / 16);

        LuminanceFilter {
            uniform_buffer,
            bind_group,
            pipeline,
            offset_stride,
            threadgroup_count,
            output_view,
        }
    }

    pub fn compute(&mut self, _device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass();
        cpass.set_pipeline(&self.pipeline);
        cpass.set_bind_group(0, &self.bind_group, &[0]);
        cpass.dispatch(self.threadgroup_count.0, self.threadgroup_count.1, 1);

        cpass.set_bind_group(0, &self.bind_group, &[self.offset_stride]);
        cpass.dispatch(self.threadgroup_count.0, self.threadgroup_count.1, 1);
    }
}
