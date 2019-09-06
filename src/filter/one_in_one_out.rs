#[allow(dead_code)]
pub struct OneInOneOut {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}

#[allow(dead_code)]
impl OneInOneOut {
    pub fn new(
        device: &mut wgpu::Device, in_view: &wgpu::TextureView, out_view: &wgpu::TextureView,
        uniform_buffer: wgpu::Buffer, buffer_range: wgpu::BufferAddress,
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
        OneInOneOut { bind_group_layout, bind_group, uniform_buffer }
    }
}
