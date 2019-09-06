#[allow(dead_code)]
pub struct OneInOneOut {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub pipeline: wgpu::ComputePipeline,
    threadgroup_count: (u32, u32),
}

#[allow(dead_code)]
impl OneInOneOut {
    pub fn new(
        device: &mut wgpu::Device, in_view: &wgpu::TextureView, out_view: &wgpu::TextureView,
        extent: wgpu::Extent3d, uniform_buffer: wgpu::Buffer, buffer_range: wgpu::BufferAddress,
        shader_name: &str,
    ) -> Self {
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
        let bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buffer,
                        range: 0..buffer_range,
                    },
                },
                wgpu::Binding { binding: 1, resource: wgpu::BindingResource::TextureView(in_view) },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(out_view),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let shader =
            idroid::shader::Shader::new_by_compute(shader_name, device, env!("CARGO_MANIFEST_DIR"));
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: &pipeline_layout,
            compute_stage: shader.cs_stage(),
        });

        let threadgroup_count = ((extent.width + 15) / 16, (extent.height + 15) / 16);

        OneInOneOut { bind_group_layout, bind_group, uniform_buffer, pipeline, threadgroup_count }
    }

    pub fn compute(&mut self, _device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass();
        cpass.set_pipeline(&self.pipeline);
        cpass.set_bind_group(0, &self.bind_group, &[0]);
        cpass.dispatch(self.threadgroup_count.0, self.threadgroup_count.1, 1);
    }
}
