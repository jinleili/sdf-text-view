use crate::filter::OneInOneOut;
use crate::PicInfoUniform;
use std::ops::{Deref, DerefMut};
use wgpu::util::DeviceExt;
use zerocopy::AsBytes;

#[allow(dead_code)]
pub struct LuminanceFilter {
    one_in_out: OneInOneOut,
    pub output_view: wgpu::TextureView,
}

#[allow(dead_code)]
impl LuminanceFilter {
    pub fn new(
        device: &mut wgpu::Device, _encoder: &mut wgpu::CommandEncoder,
        src_view: &wgpu::TextureView, extent: wgpu::Extent3d,
    ) -> Self {
        let offset_stride = std::mem::size_of::<PicInfoUniform>() as wgpu::BufferAddress;
        let uniform_size = offset_stride * 1;
        let output_view = idroid::load_texture::empty(
            device,
            wgpu::TextureFormat::R32Float,
            extent,
            Some(wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::STORAGE),
        )
        .1;

        let one_in_out = OneInOneOut::new(
            device,
            src_view,
            &output_view,
            extent,
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &[PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 0, 0],
                    any: [0; 60],
                }]
                .as_bytes(),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }),
            uniform_size,
            "filter/luminance",
        );
        LuminanceFilter { one_in_out, output_view }
    }
}

impl Deref for LuminanceFilter {
    type Target = OneInOneOut;
    fn deref<'a>(&'a self) -> &'a OneInOneOut {
        &self.one_in_out
    }
}

impl DerefMut for LuminanceFilter {
    fn deref_mut<'a>(&'a mut self) -> &'a mut OneInOneOut {
        &mut self.one_in_out
    }
}
