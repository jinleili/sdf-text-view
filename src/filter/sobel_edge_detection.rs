use crate::filter::OneInOneOut;
use crate::PicInfoUniform;
use idroid::texture;

#[allow(dead_code)]
pub struct SobelEdgeDetection {
    one_in_out: OneInOneOut,
    pipeline: wgpu::ComputePipeline,
    offset_stride: wgpu::BufferAddress,
    threadgroup_count: (u32, u32),
    pub output_view: wgpu::TextureView,
}

#[allow(dead_code)]
impl SobelEdgeDetection {
    pub fn new(
        device: &mut wgpu::Device, _encoder: &mut wgpu::CommandEncoder,
        src_view: &wgpu::TextureView, extent: wgpu::Extent3d,
    ) -> Self {
        let offset_stride = std::mem::size_of::<PicInfoUniform>() as wgpu::BufferAddress;
        let uniform_size = offset_stride * 1;
        let output_view = texture::empty(device, wgpu::TextureFormat::Rgba8Unorm, extent);

        let one_in_out = OneInOneOut::new(
            device,
            src_view,
            &output_view,
            device
                .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
                .fill_from_slice(&[PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 0, 0],
                    any: [0; 60],
                }]),
            uniform_size,
        );

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&one_in_out.bind_group_layout],
        });

        let shader = idroid::shader::Shader::new_by_compute(
            "filter/sobel_edge_detection",
            device,
            env!("CARGO_MANIFEST_DIR"),
        );
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: &pipeline_layout,
            compute_stage: shader.cs_stage(),
        });

        let threadgroup_count = ((extent.width + 15) / 16, (extent.height + 15) / 16);

        SobelEdgeDetection { one_in_out, pipeline, offset_stride, threadgroup_count, output_view }
    }

    pub fn compute(&mut self, _device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass();
        cpass.set_pipeline(&self.pipeline);
        cpass.set_bind_group(0, &self.one_in_out.bind_group, &[0]);
        cpass.dispatch(self.threadgroup_count.0, self.threadgroup_count.1, 1);
    }
}
