use std::iter;
use wgpu::{CommandEncoder, Device, PipelineLayout, RenderPipeline, TextureFormat, TextureView};
use crate::WGPUConstructor;
use crate::WGPU::RenderPipelineUtility;
use crate::WGPU::MaterialManager::{Material, MaterialManager};

pub struct WGPUManager {
    wgpu_constructor: WGPUConstructor,
    material_manager: MaterialManager,

    default_pipeline_layout: PipelineLayout,
    default_render_pipeline : RenderPipeline,
}

impl WGPUManager {
    pub fn new(wgpu_constructor: WGPUConstructor) -> Self {
        let mut material_manager = MaterialManager::new();
        let material = material_manager.load_shader(&String::from("./assets/shader/shader.wgsl"), &wgpu_constructor.device);
        let commonShader = material.unwrap();

        let pipleline_layout = RenderPipelineUtility::create_layout(&wgpu_constructor.device);
        let render_pipeline =  RenderPipelineUtility::create_pipeline(&wgpu_constructor.device, &commonShader.shader_mudule, &pipleline_layout, wgpu_constructor.config.format);

        Self {
            wgpu_constructor,
            material_manager: material_manager,
            default_pipeline_layout: pipleline_layout,
            default_render_pipeline: render_pipeline,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.wgpu_constructor.resize(new_size);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>{
        let output = self.wgpu_constructor.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = self.create_encoder_buffer(&view);

        self.wgpu_constructor.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn create_encoder_buffer(&mut self, view : &TextureView) -> CommandEncoder {
        let mut encoder = self.wgpu_constructor.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.default_render_pipeline); // 2.
            render_pass.draw(0..3, 0..1); // 3.
        }

        return encoder;
    }
}
