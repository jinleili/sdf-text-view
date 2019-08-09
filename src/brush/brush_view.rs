use crate::SurfaceView;

use uni_view::{AppView, GPUContext};

use super::{touch_points, InkDeposite, PaperNode};

pub struct BrushView {
    app_view: AppView,
    // 沉积层
    ink_deposite: InkDeposite,
    paper_node: PaperNode,
    // 最终呈现层
    present_node: PaperNode,

    step_offset: usize,
    gap: u32,
}

impl BrushView {
    pub fn new(app_view: AppView) -> Self {
        let mut app_view = app_view;
        let mut encoder =
            app_view.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        let ink_deposite = InkDeposite::new(&app_view.sc_desc, &mut app_view.device, &mut encoder);

        let (texture_view, _texture_extent, _sampler) =
            crate::texture::from_file("brush/paper1.jpg", &mut app_view.device, &mut encoder);

        let paper_node = super::PaperNode::new(
            &app_view.sc_desc,
            &mut app_view.device,
            Some(&texture_view),
            "none",
        );

        let present_node = super::PaperNode::new(
            &app_view.sc_desc,
            &mut app_view.device,
            Some(&ink_deposite.canvas_texture_view),
            "brush/present_filter",
        );

        // 纹理啥的 submit 之后才能真正创建好
        let init_command_buf = encoder.finish();
        app_view.device.get_queue().submit(&[init_command_buf]);

        BrushView { app_view, ink_deposite, paper_node, present_node, step_offset: 0, gap: 130 }
    }

    fn step_frame_data(&mut self, encoder: &mut wgpu::CommandEncoder) {
        // self.step_offset = 0;
        // if self.step_offset > 18 {
        //     return;
        // }
        if touch_points.len() == self.step_offset {
            return;
        }
        self.gap += 1;
        if self.gap < 4 {
            return;
        }

        self.gap = 0;

        let mut p = touch_points[self.step_offset];
        // 转换成屏幕中心为原点的坐标，方向旋转变换的计算
        p[0] *= self.app_view.scale_factor;
        p[1] *= self.app_view.scale_factor;
        p[0] -= self.app_view.sc_desc.width as f32 / 2.0;
        p[1] -= self.app_view.sc_desc.height as f32 / 2.0;

        if self.step_offset == 0 {
            self.ink_deposite.touch_start(&mut self.app_view.device, [p[0], p[1]]);
        // self.ink_deposite.begin_render_pass(encoder);
        } else if (self.step_offset + 1) < touch_points.len() {
            self.ink_deposite.touch_moved(&mut self.app_view.device, [p[0], p[1]]);
        } else {
            self.ink_deposite.touch_end(&mut self.app_view.device, [p[0], p[1]]);
        }

        self.ink_deposite.begin_render_pass(encoder);

        self.step_offset += 1;
    }
}

impl SurfaceView for BrushView {
    fn resize(&mut self) {
        self.app_view.update_swap_chain();
        self.ink_deposite.resize(&self.app_view.sc_desc, &mut self.app_view.device);
    }

    fn update(&mut self, _event: wgpu::winit::WindowEvent) {}
    fn touch_moved(&mut self, position: crate::math::Position) {}

    fn enter_frame(&mut self) {
        let mut encoder = self
            .app_view
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            self.step_frame_data(&mut encoder);

            let frame = self.app_view.swap_chain.get_next_texture();
            {
                self.paper_node.begin_render_pass(&frame.view, &mut encoder, wgpu::LoadOp::Clear);
                self.present_node.begin_render_pass(&frame.view, &mut encoder, wgpu::LoadOp::Load);
            }
            self.app_view.device.get_queue().submit(&[encoder.finish()]);
        }
    }
}
