use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use wgpu::{Buffer, CommandBuffer, CommandEncoder, Device, PipelineLayout, RenderPipeline, SurfaceTexture, TextureFormat, TextureView};
use crate::{ObjectDataDefineJSON, VertexBufferManager, WGPUConstructor};
use crate::Type::ObjectBufferType::Vertex;
use crate::WGPU::RenderPipelineUtility;
use crate::WGPU::MaterialManager::{Material, MaterialManager};

pub struct WGPUManager {
    pub wgpu_constructor: WGPUConstructor,
    material_manager: MaterialManager,

    default_pipeline_layout: PipelineLayout,
    default_render_pipeline : RenderPipeline,
}

impl WGPUManager {
    pub fn new(wgpu_constructor: WGPUConstructor) -> Self {
        let mut material_manager = MaterialManager::new();
        let material = material_manager.load_shader(&String::from("./assets/shader/shader.wgsl"), &wgpu_constructor.device);
        let commonShader = material.unwrap();

        let mut pipleline_layout;
        let mut render_pipeline;
        {
            pipleline_layout = RenderPipelineUtility::create_layout(&wgpu_constructor.device);
            render_pipeline =  RenderPipelineUtility::create_pipeline(&wgpu_constructor.device,
                                                                          &commonShader.shader_mudule, &pipleline_layout, wgpu_constructor.config.format,
                                                                          Vertex::desc());
        }

        // let mut vertex_buffers = VertexBufferManager::new(&unwrap_wgpu_cont.device);
        // vertex_buffers.insert_json_data(&object_data_json.unwrap());


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

    pub fn render(&mut self, vertex_manager: &VertexBufferManager) -> Result<(), wgpu::SurfaceError>{
        let output = self.wgpu_constructor.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut command_buffers: Vec<CommandBuffer> = Vec::new();
        let vertex_buffers = vertex_manager.get_all_vertex_buffer();

        for x in vertex_buffers {
            let vb_value = x.borrow();
            let command_buffer =self.create_encoder_buffer(&view,
                                                           vb_value.number_of_vertices,
                                                           &vb_value.buffer_data).finish();
            command_buffers.push(command_buffer);
        }

        self.wgpu_constructor.queue.submit(command_buffers);
        output.present();

        Ok(())
    }

    pub fn create_encoder_buffer(&mut self, view : &TextureView, vertetice_number : u32, vertex_buffer : &Buffer) -> CommandEncoder {
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
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(0..vertetice_number, 0..1);

        }

        return encoder;
    }
}
