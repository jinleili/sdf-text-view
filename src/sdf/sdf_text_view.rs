use crate::texture;
use crate::utils::HUD;
use crate::SurfaceView;

use super::{SDFComputeNode, SDFRenderNode};
use uni_view::{AppView, GPUContext};

pub struct SDFTextView {
    app_view: AppView,
    hud: HUD,
    compute_node: SDFComputeNode,
    render_node: SDFRenderNode,
    need_cal_sdf: bool,
    need_draw: bool,
    draw_count: u8,
}

impl SDFTextView {
    pub fn new(app_view: AppView) -> Self {
        let mut app_view = app_view;

        let mut encoder =
            app_view.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        // Create the texture
        let (texture_view, texture_extent, _sampler) = texture::from_file_and_usage_write(
            "large.png",
            &mut app_view.device,
            &mut encoder,
            true,
            true,
        );

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

        let mut instance = SDFTextView {
            app_view,
            hud,
            compute_node,
            render_node,
            need_cal_sdf: true,
            need_draw: true,
            draw_count: 0,
        };
        // invoke resize to update mvp matrix
        instance.resize();

        instance
    }
}

impl SurfaceView for SDFTextView {
    fn touch_moved(&mut self, _position: crate::math::Position) {}

    fn resize(&mut self) {
        println!("resize()--");
        self.app_view.update_swap_chain();
        self.render_node.update_scale(&self.app_view.sc_desc, &mut self.app_view.device, 1.0);
        self.need_draw = true;
    }

    fn scale(&mut self, scale: f32) {
        self.render_node.update_scale(&self.app_view.sc_desc, &mut self.app_view.device, scale);
        self.need_draw = true;
    }

    fn enter_frame(&mut self) {
        if self.need_draw == false {
            return;
        }
        let mut encoder = self
            .app_view
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            if self.need_cal_sdf {
                self.hud.start_frame_timer();
                self.compute_node.compute(&mut self.app_view.device, &mut encoder);
                self.need_cal_sdf = false;
                println!("sdf cost: {:?}", self.hud.stop_frame_timer());
                // encoder.copy_buffer_to_buffer(
                //     &self.compute_node.sdf_buffer,
                //     0,
                //     &self.compute_node.staging_buffer,
                //     0,
                //     18 * 22 * 4,
                // );
            }

            let frame = self.app_view.swap_chain.get_next_texture();
            {
                self.render_node.begin_render_pass(&frame, &mut encoder, &mut self.app_view.device);

                // draw for all swap_chain frame textures, then, stop to draw frame until resize() or rotate() fn called.
                self.draw_count += 1;
                if self.draw_count == 3 {
                    self.need_draw = false;
                    self.draw_count = 0;
                }
            }
            self.app_view.device.get_queue().submit(&[encoder.finish()]);

            // self.compute_node.staging_buffer.map_read_async(
            //     0,
            //     18 * 22 * 4,
            //     |result: wgpu::BufferMapAsyncResult<&[f32]>| {
            //         if let Ok(mapping) = result {
            //             println!("Times: {:?}", mapping.data);
            //         }
            //     },
            // );
        }
    }
}
