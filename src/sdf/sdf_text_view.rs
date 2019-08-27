use crate::texture;
use crate::utils::{MVPUniform, HUD};
use crate::SurfaceView;

use uni_view::{AppView, GPUContext};

use super::{SDFComputeNode, SDFRenderNode};

// use nalgebra_glm as glm;

pub struct SDFTextView {
    app_view: AppView,
    hud: HUD,
    compute_node: SDFComputeNode,
    render_node: SDFRenderNode,
    need_cal_sdf: bool,
}

impl SDFTextView {
    pub fn new(app_view: AppView) -> Self {
        let mut app_view = app_view;

        let mut encoder =
            app_view.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        // Create the texture
        let (texture_view, texture_extent, _sampler) = texture::from_file_and_usage_write(
            "math2.png",
            &mut app_view.device,
            &mut encoder,
            true,
            true,
        );

        print!("extent: {:?}", texture_extent);

        let compute_node =
            SDFComputeNode::new(&mut app_view.device, &mut encoder, &texture_view, texture_extent);
        // compute_node.compute(&mut app_view.device, &mut encoder);

        let render_node = SDFRenderNode::new(
            &app_view.sc_desc,
            &mut app_view.device,
            &texture_view,
            texture_extent,
        );
        app_view.device.get_queue().submit(&[encoder.finish()]);

        let hud = HUD::new();

        SDFTextView { app_view, hud, compute_node, render_node, need_cal_sdf: true }
    }
}

impl SurfaceView for SDFTextView {
    fn update(&mut self, _event: winit::event::WindowEvent) {
        //empty
    }

    fn touch_moved(&mut self, _position: crate::math::Position) {}

    fn resize(&mut self) {
        self.app_view.update_swap_chain();
        self.render_node.update_scale(&self.app_view.sc_desc, &mut self.app_view.device, 1.0);
    }

    fn enter_frame(&mut self) {
        let mut encoder = self
            .app_view
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            if self.need_cal_sdf {
                self.hud.start_frame_timer();
                self.compute_node.compute(&mut self.app_view.device, &mut encoder);
                self.need_cal_sdf = false;
                println!("sdf time: {:?}", self.hud.stop_frame_timer());
            }

            let frame = self.app_view.swap_chain.get_next_texture();
            {
                self.render_node.begin_render_pass(&frame, &mut encoder, &mut self.app_view.device);
            }
            self.app_view.device.get_queue().submit(&[encoder.finish()]);
        }
    }
}
