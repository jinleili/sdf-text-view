use idroid::{texture, utils::HUD, SurfaceView};

use super::{
    clear_node::ClearColorNode, compute_node::SDFComputeNode, filter::CannyEdgeDetection,
    filter::GaussianBlurFilter, filter::LuminanceFilter, render_node::SDFRenderNode,
};
use uni_view::{fs::FileSystem, AppView, GPUContext};

pub struct SDFTextView {
    app_view: AppView,
    hud: HUD,
    image: Option<String>,
    compute_node: Option<SDFComputeNode>,
    render_node: Option<SDFRenderNode>,
    edge_detection: Option<CannyEdgeDetection>,
    clear_color_node: ClearColorNode,
    need_clear_color: bool,
    clear_count: u8,
    need_cal_sdf: bool,
    need_draw: bool,
    draw_count: u8,
    need_auto_detect: bool,
}

impl SDFTextView {
    pub fn new(app_view: AppView) -> Self {
        let mut app_view = app_view;
        let hud = HUD::new();
        let clear_color_node = ClearColorNode::new(&app_view.sc_desc, &mut app_view.device);
        let instance = SDFTextView {
            app_view,
            hud,
            image: None,
            compute_node: None,
            render_node: None,
            edge_detection: None,
            clear_color_node,
            need_clear_color: true,
            need_cal_sdf: false,
            need_draw: false,
            draw_count: 0,
            clear_count: 0,
            need_auto_detect: false,
        };
        instance
    }

    pub fn bundle_image(&mut self, path: String, need_auto_detect: bool) {
        self.need_clear_color = false;
        self.image = Some(path);
        self.need_draw = true;
        self.need_auto_detect = need_auto_detect;
    }

    fn create_nodes(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let fs = FileSystem::new(env!("CARGO_MANIFEST_DIR"));
        let path = fs.get_texture_file_path(&self.image.as_ref().unwrap());
        let (texture_view, texture_extent, _sampler) = texture::from_path(
            path,
            &mut self.app_view.device,
            encoder,
            true,
            if self.need_auto_detect { false } else { true },
        );

        let output_view = if self.need_auto_detect {
            let edge_detection = CannyEdgeDetection::new(
                &mut self.app_view.device,
                encoder,
                &texture_view,
                texture_extent,
            );
            self.edge_detection = Some(edge_detection);

            &self.edge_detection.as_ref().unwrap().output_view
        } else {
            &texture_view
        };

        let compute_node =
            SDFComputeNode::new(&mut self.app_view.device, encoder, output_view, texture_extent);

        let mut render_node = SDFRenderNode::new(
            &self.app_view.sc_desc,
            &mut self.app_view.device,
            output_view,
            texture_extent,
        );
        // update mvp matrix
        render_node.update_scale(&self.app_view.sc_desc, &mut self.app_view.device, 1.0);

        self.compute_node = Some(compute_node);
        self.render_node = Some(render_node);
        self.need_cal_sdf = true;
        self.draw_count = 0;
        self.need_draw = true;
    }
}

impl SurfaceView for SDFTextView {
    fn touch_moved(&mut self, _position: idroid::math::Position) {}

    fn resize(&mut self) {
        println!("resize()--");
        if let Some(render_node) = &mut self.render_node {
            render_node.update_scale(&self.app_view.sc_desc, &mut self.app_view.device, 1.0);
            self.app_view.update_swap_chain();
            self.need_draw = true;
            self.enter_frame();
        }
    }

    fn scale(&mut self, scale: f32) {
        if let Some(render_node) = &mut self.render_node {
            render_node.update_scale(&self.app_view.sc_desc, &mut self.app_view.device, scale);
            self.need_draw = true;
        }
    }

    fn enter_frame(&mut self) {
        if self.need_draw == false {
            if self.need_clear_color && self.clear_count < 3 {
                let frame = self.app_view.swap_chain.get_next_texture();
                {
                    self.clear_color_node.clear_color(&frame, &mut self.app_view.device);
                    self.clear_count += 1;
                }
            }
            return;
        }

        let mut encoder = self
            .app_view
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        let _ = match (&mut self.compute_node, &mut self.render_node) {
            (Some(compute_node), Some(render_node)) => {
                if self.need_cal_sdf {
                    self.hud.start_frame_timer();
                    if self.need_auto_detect {
                        self.edge_detection
                            .as_mut()
                            .unwrap()
                            .compute(&mut self.app_view.device, &mut encoder);
                    }
                    compute_node.compute(&mut self.app_view.device, &mut encoder);
                    self.need_cal_sdf = false;
                    println!("sdf cost: {:?}", self.hud.stop_frame_timer());
                }

                let frame = self.app_view.swap_chain.get_next_texture();
                {
                    render_node.begin_render_pass(&frame, &mut encoder);
                    // draw for all swap_chain frame textures, then, stop to draw frame until resize() or rotate() fn called.
                    self.draw_count += 1;
                    if self.draw_count == 3 {
                        self.need_draw = false;
                        self.draw_count = 0;
                    }
                }
                self.app_view.device.get_queue().submit(&[encoder.finish()]);
            }
            (_, _) => {
                self.create_nodes(&mut encoder);
                self.app_view.device.get_queue().submit(&[encoder.finish()]);
            }
        };

        // self.app_view.device.get_queue().submit(&[encoder.finish()]);
    }
}
