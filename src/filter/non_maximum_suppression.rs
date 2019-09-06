use crate::filter::OneInOneOut;
use crate::PicInfoUniform2;
use std::ops::{ Deref, DerefMut };

#[allow(dead_code)]
pub struct NonMaximumSuppression {
    one_in_out: OneInOneOut,
}

#[allow(dead_code)]
impl NonMaximumSuppression {
    pub fn new(
        device: &mut wgpu::Device, _encoder: &mut wgpu::CommandEncoder,
        in_view: &wgpu::TextureView, out_view: &wgpu::TextureView, extent: wgpu::Extent3d,
    ) -> Self {
        let offset_stride = std::mem::size_of::<PicInfoUniform2>() as wgpu::BufferAddress;
        let uniform_size = offset_stride * 1;

        let one_in_out = OneInOneOut::new(
            device,
            in_view,
            out_view,
            extent,
            device
                .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
                .fill_from_slice(&[PicInfoUniform2 {
                    info: [extent.width as i32, extent.height as i32, 0, 0],
                    threshold: [0.0, 0.4, 0.0, 0.0],
                    any: [0; 56],
                }]),
            uniform_size,
            "filter/non_maximum_suppression",
        );

        NonMaximumSuppression { one_in_out }
    }
}

impl Deref for NonMaximumSuppression {
    type Target = OneInOneOut;
    fn deref<'a>(&'a self) -> &'a OneInOneOut {
        &self.one_in_out
    }
}

impl DerefMut for NonMaximumSuppression {
    fn deref_mut<'a>(&'a mut self) -> &'a mut OneInOneOut {
        &mut self.one_in_out
    }
}
