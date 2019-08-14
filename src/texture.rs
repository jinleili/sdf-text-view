use wgpu::{Extent3d, Sampler, TextureView};
use image::GenericImageView;

#[allow(dead_code)]
pub fn from_file(
    image_name: &str, device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder,
) -> (TextureView, Extent3d, Sampler) {
    self::from_file_and_usage_write(image_name, device, encoder, false, false)
}

// is_gray_pic: 是否为单通道灰度纹理
#[allow(dead_code)]
pub fn from_file_and_usage_write(
    image_name: &str, device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder,
    usage_write: bool, is_gray_pic: bool
) -> (TextureView, Extent3d, Sampler) {
    // 动态加载本地文件
    let path = uni_view::fs::FileSystem::get_texture_file_path(image_name);

    let image_bytes = match std::fs::read(&path) {
        Ok(code) => code,
        Err(e) => panic!("Unable to read {:?}: {:?}", path, e),
    };

    let img_load = image::load_from_memory(&image_bytes).expect("Failed to load image.");
    let img_raw = if is_gray_pic { img_load.to_luma().into_raw() } else { img_load.to_rgba().into_raw() };

    let (width, height) = img_load.dimensions();
    let texture_extent = wgpu::Extent3d { width: width, height: height, depth: 1 };
    let usage = if usage_write {
        wgpu::TextureUsage::TRANSFER_DST
            | wgpu::TextureUsage::SAMPLED
            | wgpu::TextureUsage::WRITE_ALL
    } else {
        wgpu::TextureUsage::TRANSFER_DST | wgpu::TextureUsage::SAMPLED
    };
    let (format, channel_count) = if is_gray_pic { 
            (wgpu::TextureFormat::R8Unorm, 1)
        } else { 
            (wgpu::TextureFormat::Rgba8Unorm, 4)
        };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_extent,
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: format,
        usage: usage,
    });
    let texture_view = texture.create_default_view();

    let texels: Vec<u8> = img_raw;
    let temp_buf = device
        .create_buffer_mapped(texels.len(), wgpu::BufferUsage::TRANSFER_SRC)
        .fill_from_slice(&texels);
    encoder.copy_buffer_to_texture(
        wgpu::BufferCopyView {
            buffer: &temp_buf,
            offset: 0,
            row_pitch: channel_count * width,
            image_height: height,
        },
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            array_layer: 0,
            origin: wgpu::Origin3d { x: 0.0, y: 0.0, z: 0.0 },
        },
        texture_extent,
    );

    (texture_view, texture_extent, default_sampler(device))
}

#[allow(dead_code)]
pub fn from_buffer_and_usage_write(
    buffer: &wgpu::Buffer, device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder,
    width: u32, height: u32, pixel_size: u32, usage_write: bool,
) -> (TextureView, Extent3d, Sampler) {
    let texture_extent = wgpu::Extent3d { width: width, height: height, depth: 1 };
    let usage = if usage_write {
        wgpu::TextureUsage::TRANSFER_DST
            | wgpu::TextureUsage::SAMPLED
            | wgpu::TextureUsage::WRITE_ALL
    } else {
        wgpu::TextureUsage::TRANSFER_DST | wgpu::TextureUsage::SAMPLED
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_extent,
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba32Float,
        usage: usage,
    });
    let texture_view = texture.create_default_view();

    // BufferCopyView 必须 >= TextureCopyView
    encoder.copy_buffer_to_texture(
        wgpu::BufferCopyView {
            buffer: buffer,
            offset: 0,
            row_pitch: pixel_size * width,
            image_height: height,
        },
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            array_layer: 0,
            origin: wgpu::Origin3d { x: 0.0, y: 0.0, z: 0.0 },
        },
        texture_extent,
    );

    (texture_view, texture_extent, default_sampler(device))
}

// empty texture as a OUTPUT_ATTACHMENT
#[allow(dead_code)]
pub fn empty(
    device: &mut wgpu::Device, format: wgpu::TextureFormat, extent: Extent3d,
) -> TextureView {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: extent,
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: format,
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT
            | wgpu::TextureUsage::TRANSFER_DST
            | wgpu::TextureUsage::SAMPLED
            | wgpu::TextureUsage::WRITE_ALL,
    });
    let texture_view = texture.create_default_view();
    texture_view
}

#[allow(dead_code)]
pub fn empty_view(device: &mut wgpu::Device, width: u32, height: u32) -> TextureView {
    crate::texture::empty(
        device,
        wgpu::TextureFormat::Bgra8Unorm,
        wgpu::Extent3d { width: width, height: height, depth: 1 },
    )
}

// 32位浮点纹理
#[allow(dead_code)]
pub fn empty_f32_view(device: &mut wgpu::Device, width: u32, height: u32) -> TextureView {
    crate::texture::empty(
        device,
        wgpu::TextureFormat::Rgba32Float,
        wgpu::Extent3d { width: width, height: height, depth: 1 },
    )
}

#[allow(dead_code)]
pub fn default_sampler(device: &wgpu::Device) -> Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare_function: wgpu::CompareFunction::Always,
    })
}

// 双线性插值
// https://vulkan-tutorial.com/Texture_mapping/Image_view_and_sampler
#[allow(dead_code)]
pub fn bilinear_sampler(device: &wgpu::Device) -> Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare_function: wgpu::CompareFunction::Always,
    })
}
