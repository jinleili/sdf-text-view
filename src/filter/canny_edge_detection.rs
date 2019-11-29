use crate::filter::{GaussianBlurFilter, LuminanceFilter, OneInOneOut};
use crate::{PicInfoUniform, PicInfoUniform2};
use zerocopy::AsBytes;

#[allow(dead_code)]
pub struct CannyEdgeDetection {
    pub output_view: wgpu::TextureView,
    luminance_filter: LuminanceFilter,
    blur_filter: GaussianBlurFilter,
    sobel_edge_detection: OneInOneOut,
    non_maximum_suppression: OneInOneOut,
    weak_pixel_inclusion: OneInOneOut,
}

#[allow(dead_code)]
impl CannyEdgeDetection {
    pub fn new(
        device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder,
        src_view: &wgpu::TextureView, extent: wgpu::Extent3d,
    ) -> Self {
        let output_view = idroid::texture::empty(device, wgpu::TextureFormat::R8Unorm, extent);

        let luminance_filter = LuminanceFilter::new(device, encoder, src_view, extent);
        let blur_filter = GaussianBlurFilter::new(
            device,
            encoder,
            &luminance_filter.output_view,
            &output_view,
            extent,
            true,
        );

        let offset_stride: wgpu::BufferAddress = 256;
        let uniform_size = offset_stride * 1;

        let sobel_output_view =
            idroid::texture::empty(device, wgpu::TextureFormat::Rgba8Unorm, extent);
        let sobel_edge_detection = OneInOneOut::new(
            device,
            &output_view,
            &sobel_output_view,
            extent,
            device.create_buffer_with_data(
                &[PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 0, 0],
                    any: [0; 60],
                }]
                .as_bytes(),
                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            ),
            uniform_size,
            "filter/sobel_edge_detection",
        );

        let non_maximum_suppression = OneInOneOut::new(
            device,
            &sobel_output_view,
            &output_view,
            extent,
            device.create_buffer_with_data(
                &[PicInfoUniform2 {
                    info: [extent.width as i32, extent.height as i32, 0, 0],
                    threshold: [0.1, 0.4, 0.0, 0.0],
                    any: [0; 56],
                }]
                .as_bytes(),
                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            ),
            uniform_size,
            "filter/non_maximum_suppression",
        );

        let weak_pixel_inclusion = OneInOneOut::new(
            device,
            &luminance_filter.output_view,
            &output_view,
            extent,
            device.create_buffer_with_data(
                &[PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 0, 0],
                    any: [0; 60],
                }]
                .as_bytes(),
                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            ),
            uniform_size,
            "filter/weak_pixel_inclusion",
        );
        CannyEdgeDetection {
            output_view,
            luminance_filter,
            blur_filter,
            sobel_edge_detection,
            non_maximum_suppression,
            weak_pixel_inclusion,
        }
    }

    pub fn compute(&mut self, device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        self.luminance_filter.compute(device, encoder);
        self.blur_filter.compute(device, encoder);
        self.sobel_edge_detection.compute(device, encoder);
        self.non_maximum_suppression.compute(device, encoder);
        // self.weak_pixel_inclusion.compute(device, encoder);
    }
}
