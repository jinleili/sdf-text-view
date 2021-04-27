use idroid::{math::TouchPoint, node::BufferlessFullscreenNode, utils::HUD, SurfaceView};

use super::{compute_node::SDFComputeNode, filter::CannyEdgeDetection, render_node::SDFRenderNode};
use uni_view::{fs::FileSystem, AppView, GPUContext};

pub struct SDFTextView {
    pub app_view: AppView,
    hud: HUD,
    image: Option<String>,
    compute_node: Option<SDFComputeNode>,
    render_node: Option<SDFRenderNode>,
    edge_detection: Option<CannyEdgeDetection>,
    clear_color_node: BufferlessFullscreenNode,
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

        let shader = idroid::shader2::create_shader_module(&app_view.device, "clear_color", None);
        let clear_color_node = BufferlessFullscreenNode::new(&app_view, vec![], vec![], &shader);

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
        let (_, texture_view, texture_extent) = idroid::load_texture::into_format_r32float(
            &self.image.as_ref().unwrap(),
            &mut self.app_view,
            wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::STORAGE,
        );

        let src_view = if self.need_auto_detect {
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

        let output_view = idroid::load_texture::empty(
            &mut self.app_view.device,
            wgpu::TextureFormat::R32Float,
            texture_extent,
            Some(wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::STORAGE),
        )
        .1;

        let compute_node = SDFComputeNode::new(
            &mut self.app_view.device,
            encoder,
            src_view,
            &output_view,
            texture_extent,
        );

        let mut render_node =
            SDFRenderNode::new(&self.app_view, &self.app_view.device, &output_view, texture_extent);
        // update mvp matrix
        render_node.update_scale(&self.app_view.sc_desc, &mut self.app_view.device, encoder, 1.0);

        self.compute_node = Some(compute_node);
        self.render_node = Some(render_node);
        self.need_cal_sdf = true;
        self.draw_count = 0;
        self.need_draw = true;
    }
}

impl SurfaceView for SDFTextView {
    fn touch_start(&mut self, _point: TouchPoint) {}
    fn touch_moved(&mut self, _point: TouchPoint) {}
    fn touch_end(&mut self, _point: TouchPoint) {}

    fn resize(&mut self) {
        if let Some(render_node) = &mut self.render_node {
            let mut encoder = self
                .app_view
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            render_node.update_scale(
                &self.app_view.sc_desc,
                &mut self.app_view.device,
                &mut encoder,
                1.0,
            );
            self.app_view.queue.submit(Some(encoder.finish()));

            self.app_view.update_swap_chain();
            self.need_draw = true;
            self.enter_frame();
        }
    }

    fn pintch_start(&mut self, _location: idroid::math::TouchPoint, _scale: f32) {}
    fn pintch_changed(&mut self, _location: idroid::math::TouchPoint, scale: f32) {
        if let Some(render_node) = &mut self.render_node {
            let mut encoder = self
                .app_view
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            render_node.update_scale(
                &self.app_view.sc_desc,
                &mut self.app_view.device,
                &mut encoder,
                scale,
            );
            self.app_view.queue.submit(Some(encoder.finish()));
            self.need_draw = true;
        }
    }

    fn enter_frame(&mut self) {
        println!("enter_frame...");

        if self.need_draw == false {
            if self.need_clear_color && self.clear_count < 3 {
                let frame = self
                    .app_view
                    .swap_chain
                    .get_current_frame()
                    .expect("swap_chain.get_next_texture() timeout");
                {
                    let mut encoder = self
                        .app_view
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    self.clear_color_node.draw(
                        &frame.output.view,
                        &mut encoder,
                        wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    );
                    self.app_view.queue.submit(Some(encoder.finish()));
                    self.clear_count += 1;
                }
            }
            println!("return...");

            return;
        }

        let mut encoder = self
            .app_view
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        match (&mut self.compute_node, &mut self.render_node) {
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
                println!("draw...");
                let frame = self
                    .app_view
                    .swap_chain
                    .get_current_frame()
                    .expect("swap_chain.get_next_texture() timeout");
                {
                    render_node.begin_render_pass(&frame, &mut encoder);
                    // draw for all swap_chain frame textures, then, stop to draw frame until resize() or rotate() fn called.
                    self.draw_count += 1;
                    if self.draw_count == 3 {
                        self.need_draw = false;
                        self.draw_count = 0;
                    }
                }
                self.app_view.queue.submit(Some(encoder.finish()));
            }
            (..) => {
                self.create_nodes(&mut encoder);
                self.app_view.queue.submit(Some(encoder.finish()));
            }
        };

        // self.app_view.device.get_queue().submit(Some(encoder.finish()));
    }
}

impl std::ops::Deref for SDFTextView {
    type Target = AppView;
    fn deref<'a>(&'a self) -> &'a AppView {
        &self.app_view
    }
}

impl std::ops::DerefMut for SDFTextView {
    fn deref_mut<'a>(&'a mut self) -> &'a mut AppView {
        &mut self.app_view
    }
}
