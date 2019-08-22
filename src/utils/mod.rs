pub mod depth_stencil;
pub mod matrix_helper;

pub fn clear_color() -> wgpu::Color {
    wgpu::Color { r: 0.25, g: 0.25, b: 0.3, a: 1.0 }
}

#[allow(dead_code)]
pub fn black_color() -> wgpu::Color {
    wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
}

// 混合：https://vulkan.lunarg.com/doc/view/1.0.26.0/linux/vkspec.chunked/ch26s01.html
#[allow(dead_code)]
pub fn alpha_blend() -> wgpu::BlendDescriptor {
    wgpu::BlendDescriptor {
        src_factor: wgpu::BlendFactor::One,
        dst_factor: wgpu::BlendFactor::Zero,
        operation: wgpu::BlendOperation::Add,
    }
}

#[allow(dead_code)]
pub fn color_blend() -> wgpu::BlendDescriptor {
    wgpu::BlendDescriptor {
        src_factor: wgpu::BlendFactor::SrcAlpha,
        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
        operation: wgpu::BlendOperation::Add,
    }
}

// alpha 颜色混合的一种常用设置
// https://www.cnblogs.com/heitao/p/6974203.html
// src_factor：表示的是当前值的因子，dst_factor：表示缓冲区旧值的混合因子
// finalColor.rgb = newAlpha * newColor + (1 - newAlpha) * oldColor;
// finalColor.a = newAlpha.a;
#[allow(dead_code)]
pub fn color_alpha_blend() -> (wgpu::BlendDescriptor, wgpu::BlendDescriptor) {
    (
        wgpu::BlendDescriptor {
            src_factor: wgpu::BlendFactor::SrcAlpha,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
        wgpu::BlendDescriptor {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::Zero,
            operation: wgpu::BlendOperation::Add,
        },
    )
}

// 简单的颜色叠加
// 原理：https://www.jianshu.com/p/6d9a3f39bb53
#[allow(dead_code)]
pub fn color_blend_over() -> (wgpu::BlendDescriptor, wgpu::BlendDescriptor) {
    (
        wgpu::BlendDescriptor {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
        wgpu::BlendDescriptor {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
    )
}

// 颜色减法：灰色可叠加成黑色
#[allow(dead_code)]
pub fn color_blend_subtract() -> (wgpu::BlendDescriptor, wgpu::BlendDescriptor) {
    (
        wgpu::BlendDescriptor {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::ReverseSubtract,
        },
        wgpu::BlendDescriptor {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
    )
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MVPUniform {
    pub mvp_matrix: [[f32; 4]; 4],
}

// 用于暂不填充数据时
#[allow(dead_code)]
pub fn empty_uniform_buffer(device: &mut wgpu::Device, size: wgpu::BufferAddress) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        size: size,
        usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    })
}

pub fn create_uniform_buffer<T>(device: &mut wgpu::Device, uniforms: T) -> wgpu::Buffer
where
    T: 'static + Copy,
{
    device
        .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
        .fill_from_slice(&[uniforms])
}

#[allow(dead_code)]
pub fn create_uniform_buffer2<T>(
    device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder, uniforms: T,
    size: wgpu::BufferAddress,
) -> wgpu::Buffer
where
    T: 'static + Copy,
{
    let staging_buffer = device
        .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
        .fill_from_slice(&[uniforms]);
    let uniform_buffer = empty_uniform_buffer(device, size);
    encoder.copy_buffer_to_buffer(&staging_buffer, 0, &uniform_buffer, 0, size);

    uniform_buffer
}

#[allow(dead_code)]
pub fn create_storage_buffer<T>(
    device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder, slice: &[T],
    size: wgpu::BufferAddress,
) -> (wgpu::Buffer, wgpu::Buffer)
where
    T: 'static + Copy,
{
    // store buffer 不能直接创建并填充数据？
    let staging_buffer = device
        .create_buffer_mapped(
            slice.len(),
            wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC,
        )
        .fill_from_slice(slice);
    let storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        size,
        usage: wgpu::BufferUsage::STORAGE
            | wgpu::BufferUsage::COPY_DST
            | wgpu::BufferUsage::COPY_SRC,
    });
    encoder.copy_buffer_to_buffer(&staging_buffer, 0, &storage_buffer, 0, size);

    (storage_buffer, staging_buffer)
}

#[allow(dead_code)]
pub fn update_uniform<T>(device: &mut wgpu::Device, uniforms: T, destination: &wgpu::Buffer)
where
    T: 'static + Copy,
{
    let temp_buf =
        device.create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC).fill_from_slice(&[uniforms]);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
    encoder.copy_buffer_to_buffer(
        &temp_buf,
        0,
        destination,
        0,
        std::mem::size_of::<T>() as wgpu::BufferAddress,
    );
    device.get_queue().submit(&[encoder.finish()]);
}
