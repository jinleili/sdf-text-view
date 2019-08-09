use crate::geometry::plane::Plane;
use crate::texture;
use crate::utils::MVPUniform;
use crate::vertex::{Pos, PosTex};
use crate::SurfaceView;

use uni_view::{AppView, GPUContext};

use nalgebra_glm as glm;

use std::rc::Rc;

use crate::node::NoneNode;

pub struct GrayFilter {
    app_view: AppView,
    final_node: NoneNode,
}

impl GrayFilter {
    pub fn new(app_view: AppView) -> Self {
        use std::mem;
        let mut app_view = app_view;

        let mut encoder =
            app_view.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        // Create the texture
        let (texture_view, texture_extent, sampler) = texture::from_file_and_usage_write(
            "iphone.png",
            &mut app_view.device,
            &mut encoder,
            true,
        );

        // Create pipeline layout
        let bind_group_layout =
            app_view.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::SampledTexture,
                }],
            });

        let bind_group = app_view.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            }],
        });
        let pipeline_layout =
            app_view.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            });
        // Create the render pipeline
        let shader = crate::shader::Shader::new_by_compute("filter/gray", &mut app_view.device);
        let compute_pipeline =
            app_view.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                layout: &pipeline_layout,
                compute_stage: shader.cs_stage(),
            });

        let mvp = crate::matrix_helper::default_mvp(&app_view.sc_desc);
        let final_node = NoneNode::new(
            &app_view.sc_desc,
            &mut app_view.device,
            &texture_view,
            MVPUniform { mvp_matrix: mvp },
        );

        {
            let mut cpass = encoder.begin_compute_pass();
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.dispatch(texture_extent.width, texture_extent.height, 1);
        }
        app_view.device.get_queue().submit(&[encoder.finish()]);

        GrayFilter { app_view, final_node }
    }
}

impl SurfaceView for GrayFilter {
    fn update(&mut self, _event: wgpu::winit::WindowEvent) {
        //empty
    }

    fn touch_moved(&mut self, _position: crate::math::Position) {}

    fn resize(&mut self) {
        self.app_view.update_swap_chain();
    }

    fn enter_frame(&mut self) {
        let mut encoder = self
            .app_view
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let frame = self.app_view.swap_chain.get_next_texture();

            {
                self.final_node.begin_render_pass(&frame, &mut encoder);
            }

            self.app_view.device.get_queue().submit(&[encoder.finish()]);
        }
    }
}
