#[allow(dead_code)]

// 获取 depth_stencil 状态描述符
pub fn create_state_descriptor() -> wgpu::DepthStencilStateDescriptor {
    wgpu::DepthStencilStateDescriptor {
        format: wgpu::TextureFormat::D32Float,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
        stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
        stencil_read_mask: 0,
        stencil_write_mask: 0,
    }
}

#[allow(dead_code)]
pub fn create_depth_texture_view(
    sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device,
) -> wgpu::TextureView {
    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d { width: sc_desc.width, height: sc_desc.height, depth: 1 },
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::D32Float,
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
    });
    depth_texture.create_default_view()
}

#[allow(dead_code)]
// 创建 render_pass 的 depth_stencil_attachment 描述符
pub fn create_attachment_descriptor(
    depth_textue_view: &wgpu::TextureView,
) -> wgpu::RenderPassDepthStencilAttachmentDescriptor<&wgpu::TextureView> {
    wgpu::RenderPassDepthStencilAttachmentDescriptor {
        attachment: depth_textue_view,
        depth_load_op: wgpu::LoadOp::Clear,
        depth_store_op: wgpu::StoreOp::Store,
        stencil_load_op: wgpu::LoadOp::Clear,
        stencil_store_op: wgpu::StoreOp::Store,
        clear_depth: 1.0,
        clear_stencil: 0,
    }
}
