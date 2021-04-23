use crate::filter::OneInOneOut;
use crate::PicInfoUniform;
use std::ops::{Deref, DerefMut};
use wgpu::util::DeviceExt;
use zerocopy::AsBytes;

#[allow(dead_code)]
pub struct GaussianBlurFilter {
    one_in_out: OneInOneOut,
    offset_stride: wgpu::BufferAddress,
}

#[allow(dead_code)]
impl GaussianBlurFilter {
    pub fn new(
        device: &mut wgpu::Device, _encoder: &mut wgpu::CommandEncoder,
        src_view: &wgpu::TextureView, out_view: &wgpu::TextureView, extent: wgpu::Extent3d,
        only_r_channel: bool,
    ) -> Self {
        let offset_stride = std::mem::size_of::<PicInfoUniform>() as wgpu::BufferAddress;
        let uniform_size = offset_stride * 2;
        let one_in_out = OneInOneOut::new(
            device,
            src_view,
            out_view,
            extent,
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &[
                    PicInfoUniform {
                        info: [extent.width as i32, extent.height as i32, 0, 0],
                        any: [0; 60],
                    },
                    PicInfoUniform {
                        info: [extent.width as i32, extent.height as i32, 1, 0],
                        any: [0; 60],
                    },
                ]
                .as_bytes(),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }),
            uniform_size,
            if only_r_channel { "filter/gaussian_blur_r" } else { "filter/gaussian_blur_rgba" },
        );

        GaussianBlurFilter { one_in_out, offset_stride }
    }

    pub fn compute(&mut self, _device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&self.one_in_out.pipeline);
        // blur x
        cpass.set_bind_group(0, &self.one_in_out.bind_group, &[0]);
        cpass.dispatch(self.one_in_out.threadgroup_count.0, self.one_in_out.threadgroup_count.1, 1);
        // blur y
        cpass.set_bind_group(
            0,
            &self.one_in_out.bind_group,
            &[self.offset_stride as wgpu::DynamicOffset],
        );
        cpass.dispatch(self.one_in_out.threadgroup_count.0, self.one_in_out.threadgroup_count.1, 1);
    }
}

impl Deref for GaussianBlurFilter {
    type Target = OneInOneOut;
    fn deref<'a>(&'a self) -> &'a OneInOneOut {
        &self.one_in_out
    }
}

impl DerefMut for GaussianBlurFilter {
    fn deref_mut<'a>(&'a mut self) -> &'a mut OneInOneOut {
        &mut self.one_in_out
    }
}
