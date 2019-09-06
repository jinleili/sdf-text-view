use crate::filter::{
    GaussianBlurFilter, LuminanceFilter, NonMaximumSuppression, SobelEdgeDetection,
};

#[allow(dead_code)]
pub struct CannyEdgeDetection {
    pub output_view: wgpu::TextureView,
    luminance_filter: LuminanceFilter,
    blur_filter: GaussianBlurFilter,
    sobel_edge_detection: SobelEdgeDetection,
    non_maximum_suppression: NonMaximumSuppression,
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
        let sobel_edge_detection = SobelEdgeDetection::new(device, encoder, &output_view, extent);
        let non_maximum_suppression = NonMaximumSuppression::new(
            device,
            encoder,
            &sobel_edge_detection.output_view,
            &output_view,
            extent,
        );
        CannyEdgeDetection {
            output_view,
            luminance_filter,
            blur_filter,
            sobel_edge_detection,
            non_maximum_suppression,
        }
    }

    pub fn compute(&mut self, device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        self.luminance_filter.compute(device, encoder);
        self.blur_filter.compute(device, encoder);
        self.sobel_edge_detection.compute(device, encoder);
        self.non_maximum_suppression.compute(device, encoder);
    }
}
