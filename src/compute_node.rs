use crate::PicInfoUniform;
use idroid::node::{BindingGroupSettingNode, DynamicBindingGroupNode};
use wgpu::util::DeviceExt;
use zerocopy::AsBytes;

pub struct SDFComputeNode {
    setting_node: BindingGroupSettingNode,
    dynamic_node: DynamicBindingGroupNode,
    xy_pipeline: wgpu::ComputePipeline,
    x_pipeline: wgpu::ComputePipeline,
    y_pipeline: wgpu::ComputePipeline,
    offset_stride: wgpu::BufferAddress,
    threadgroup_count: (u32, u32),
    pub sdf_buffer: idroid::BufferObj,
    pub staging_buffer: wgpu::Buffer,
}

impl SDFComputeNode {
    pub fn new(
        device: &mut wgpu::Device, _encoder: &mut wgpu::CommandEncoder,
        src_view: &wgpu::TextureView, des_view: &wgpu::TextureView, extent: wgpu::Extent3d,
    ) -> Self {
        let img_size = (extent.width * extent.height) as u64;

        let offset_stride = std::mem::size_of::<PicInfoUniform>() as wgpu::BufferAddress;
        let uniform_buffer = idroid::BufferObj::create_uniforms_buffer(
            device,
            &[
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 2, 0],
                    any: [0; 60],
                },
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 0, 0],
                    any: [0; 60],
                },
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 1, 0],
                    any: [0; 60],
                },
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 0, 1],
                    any: [0; 60],
                },
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 1, 1],
                    any: [0; 60],
                },
                PicInfoUniform {
                    info: [extent.width as i32, extent.height as i32, 3, 0],
                    any: [0; 60],
                },
            ],
        );
        let dynamic_node = DynamicBindingGroupNode::new(
            device,
            vec![(&uniform_buffer, wgpu::ShaderStage::COMPUTE)],
        );

        let sdf_range = (img_size * 4) as wgpu::BufferAddress;
        let sdf_front =
            idroid::BufferObj::create_empty_storage_buffer(device, sdf_range, false, None);
        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: sdf_range,
            usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
            label: None,
            mapped_at_creation: false,
        });
        let sdf_background =
            idroid::BufferObj::create_empty_storage_buffer(device, sdf_range, false, None);

        let v_buffer =
            idroid::BufferObj::create_empty_storage_buffer(device, sdf_range, false, None);
        let z_range = ((extent.width + 1) * (extent.height + 1) * 4) as wgpu::BufferAddress;
        let z_buffer = idroid::BufferObj::create_empty_storage_buffer(device, z_range, false, None);

        let visibilitys: Vec<wgpu::ShaderStage> = [wgpu::ShaderStage::COMPUTE; 6].to_vec();
        let setting_node = BindingGroupSettingNode::new(
            device,
            vec![],
            vec![&sdf_front, &sdf_background, &v_buffer, &z_buffer],
            vec![
                (
                    src_view,
                    wgpu::TextureFormat::R32Float,
                    Some(wgpu::StorageTextureAccess::ReadOnly),
                ),
                (
                    des_view,
                    wgpu::TextureFormat::R32Float,
                    Some(wgpu::StorageTextureAccess::WriteOnly),
                ),
            ],
            vec![],
            visibilitys,
        );

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            push_constant_ranges: &[],
            bind_group_layouts: &[&setting_node.bind_group_layout, &dynamic_node.bind_group_layout],
        });

        let shader_xy = idroid::shader2::create_shader_module(device, "sdf/sdf", None);
        let xy_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: Some(&pipeline_layout),
            module: &shader_xy,
            entry_point: "main",
            label: None,
        });

        let shader_x = idroid::shader2::create_shader_module(device, "sdf/sdf_x", None);
        let x_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: Some(&pipeline_layout),
            module: &shader_x,
            entry_point: "main",
            label: None,
        });

        let shader_y = idroid::shader2::create_shader_module(device, "sdf/sdf_y", None);
        let y_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: Some(&pipeline_layout),
            module: &shader_y,
            entry_point: "main",
            label: None,
        });

        let threadgroup_count = ((extent.width + 15) / 16, (extent.height + 15) / 16);

        SDFComputeNode {
            setting_node,
            dynamic_node,
            xy_pipeline,
            x_pipeline,
            y_pipeline,
            offset_stride,
            threadgroup_count,
            staging_buffer,
            sdf_buffer: sdf_front,
        }
    }

    pub fn compute(&mut self, _device: &mut wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&self.xy_pipeline);
        cpass.set_bind_group(0, &self.setting_node.bind_group, &[]);
        cpass.set_bind_group(1, &self.dynamic_node.bind_group, &[0]);
        cpass.dispatch(self.threadgroup_count.0, self.threadgroup_count.1, 1);

        cpass.set_pipeline(&self.x_pipeline);
        // step background y
        cpass.set_bind_group(
            1,
            &self.dynamic_node.bind_group,
            &[self.offset_stride as wgpu::DynamicOffset * 3],
        );
        cpass.dispatch(self.threadgroup_count.0, 1, 1);

        // step front y
        cpass.set_bind_group(
            1,
            &self.dynamic_node.bind_group,
            &[self.offset_stride as wgpu::DynamicOffset],
        );
        cpass.dispatch(self.threadgroup_count.0, 1, 1);

        cpass.set_pipeline(&self.y_pipeline);
        // step background x
        cpass.set_bind_group(
            1,
            &self.dynamic_node.bind_group,
            &[self.offset_stride as wgpu::DynamicOffset * 4],
        );
        cpass.dispatch(1, self.threadgroup_count.1, 1);

        // step front x
        cpass.set_bind_group(
            1,
            &self.dynamic_node.bind_group,
            &[self.offset_stride as wgpu::DynamicOffset * 2],
        );
        cpass.dispatch(1, self.threadgroup_count.1, 1);

        // final output
        cpass.set_pipeline(&self.xy_pipeline);
        cpass.set_bind_group(
            1,
            &self.dynamic_node.bind_group,
            &[self.offset_stride as wgpu::DynamicOffset * 5],
        );
        cpass.dispatch(self.threadgroup_count.0, self.threadgroup_count.1, 1);
    }
}
