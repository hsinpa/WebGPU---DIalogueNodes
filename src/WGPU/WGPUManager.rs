use wgpu::{Device, TextureView};
use crate::WGPUConstructor;
use crate::WGPU::RenderPipelineManager;
use crate::WGPU::MaterialManager::{Material, MaterialManager};

pub struct WGPUManager {
    wgpu_constructor: WGPUConstructor,
    material_manager: MaterialManager,
}

impl WGPUManager {
    pub fn new(wgpu_constructor: WGPUConstructor) -> Self {
        let mut material_manager = MaterialManager::new();
        material_manager.load_shader(&String::from("./src/shader.wgsl"), &wgpu_constructor.device);

        //RenderPipelineManager::create_pipeline(&wgpu_constructor.device, )
        Self {
            wgpu_constructor,
            material_manager: material_manager
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

        Ok(())
    }

    pub fn create_encoder_buffer(&mut self, view : &TextureView) {
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

            // render_pass.set_pipeline(&self.wgpu_constructor.); // 2.
            // render_pass.draw(0..3, 0..1); // 3.
        }
    }
}
