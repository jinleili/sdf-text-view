use idroid::geometry::plane::Plane;
use idroid::{node::ImageNodeBuilder, node::ImageViewNode, MVPUniformObj};

use nalgebra_glm as glm;
use wgpu::Extent3d;
use zerocopy::{AsBytes, FromBytes};

pub struct SDFRenderNode {
    extent: Extent3d,
    scale: f32,
    view_node: ImageViewNode,
    mvp_buf: MVPUniformObj,
}

#[repr(C)]
#[derive(Clone, Copy, AsBytes, FromBytes)]
pub struct DrawUniform {
    stroke_color: [f32; 4],
    mask_n_gamma: [f32; 2],
    padding: [f32; 58],
}

impl SDFRenderNode {
    pub fn new(
        app_view: &idroid::AppView, device: &wgpu::Device, src_view: &wgpu::TextureView,
        extent: Extent3d,
    ) -> Self {
        let sampler = idroid::load_texture::bilinear_sampler(device);
        let shader_stages =
            [wgpu::ShaderStage::VERTEX, wgpu::ShaderStage::FRAGMENT, wgpu::ShaderStage::FRAGMENT]
                .to_vec();

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let mvp_buf = idroid::MVPUniformObj::new((&app_view.sc_desc).into(), device, &mut encoder);
        // Create the vertex and index buffers
        let (vertex_data, index_data) = Plane::new(1, 1).generate_vertices();

        let dynamic_buf = idroid::BufferObj::create_uniforms_buffer(
            device,
            &[
                DrawUniform {
                    stroke_color: [0.14, 0.14, 0.14, 1.0],
                    mask_n_gamma: [0.70, 0.0],
                    padding: [0.0; 58],
                },
                DrawUniform {
                    stroke_color: [0.97, 0.92, 0.80, 1.0],
                    mask_n_gamma: [0.75, 0.75],
                    padding: [0.0; 58],
                },
            ],
        );

        let shader = idroid::shader2::create_shader_module(device, "text", None);
        let builder =
            ImageNodeBuilder::new(vec![(src_view, wgpu::TextureFormat::R32Float, None)], &shader)
                .with_samplers(vec![&sampler])
                .with_vertices_and_indices((vertex_data, index_data))
                .with_shader_states(shader_stages)
                .with_uniform_buffers(vec![&mvp_buf.buffer])
                .with_dynamic_uniforms(vec![(&dynamic_buf, wgpu::ShaderStage::FRAGMENT)]);
        let view_node = builder.build(device, &mut encoder);

        app_view.queue.submit(Some(encoder.finish()));
        SDFRenderNode { extent, scale: 1.0, view_node, mvp_buf }
    }

    pub fn update_scale(
        &mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device,
        encoder: &mut wgpu::CommandEncoder, scale: f32,
    ) {
        let fovy: f32 = 75.0 / 180.0 * std::f32::consts::PI;
        let radian: glm::TVec1<f32> = glm::vec1(fovy);
        let p_matrix: glm::TMat4<f32> = glm::perspective_fov(
            radian[0],
            sc_desc.width as f32,
            sc_desc.height as f32,
            0.1,
            100.0,
        );
        let mut vm_matrix = glm::TMat4::identity();
        let sc_ratio = sc_desc.width as f32 / sc_desc.height as f32;
        let tex_ratio = self.extent.width as f32 / self.extent.height as f32;
        // maintain texture's aspect ratio
        vm_matrix = glm::scale(&vm_matrix, &glm::vec3(1.0, 1.0 / tex_ratio, 1.0));

        // when viewport's h > w,  ratio = h / w, when w > h ，ratio = 1
        let ratio = if sc_ratio < 1.0 { sc_desc.height as f32 / sc_desc.width as f32 } else { 1.0 };
        // use fovy calculate z translate distance
        let factor: f32 = (fovy / 2.0).tan();

        // full fill viewport's width or height
        let mut translate_z = -(ratio / factor);
        if sc_ratio < tex_ratio {
            if tex_ratio > 1.0 {
                translate_z /= sc_ratio * ratio;
            }
        } else {
            translate_z /= tex_ratio;
            // when tex h > w and viewport h > w, need fill the viewport's height, and the height ration is not 1.0
            if tex_ratio < 1.0 {
                translate_z /= ratio;
            };
        }
        vm_matrix = glm::translate(&vm_matrix, &glm::vec3(0.0, 0.0, translate_z));

        self.scale *= scale;
        vm_matrix = glm::scale(&vm_matrix, &glm::vec3(self.scale, self.scale, 1.0));

        let mvp: [[f32; 4]; 4] = (p_matrix * vm_matrix).into();
        self.mvp_buf.buffer.update_buffer(encoder, device, &mvp);
    }

    pub fn begin_render_pass(
        &self, frame: &wgpu::SwapChainFrame, encoder: &mut wgpu::CommandEncoder,
    ) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.output.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(idroid::utils::clear_color()),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        self.view_node.set_rpass(&mut rpass);
        self.view_node.draw_rpass_by_offset(&mut rpass, 0, 1);
        self.view_node.draw_rpass_by_offset(&mut rpass, 1, 1);
    }
}
