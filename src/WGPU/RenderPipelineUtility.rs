use wgpu::{Buffer, Device, RenderPipeline, ShaderModule, TextureFormat};
use wgpu::util::DeviceExt;
use crate::Type::ObjectBufferType::{ObjectDataDefineJSON, ObjectBufferJSON, Vertex};

pub fn create_layout(device : &Device) -> wgpu::PipelineLayout {
    return device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
}

pub fn create_pipeline(device : &Device, shader : &ShaderModule, pipeline_layout: &wgpu::PipelineLayout,
texture_format : TextureFormat, buffer_layout : wgpu::VertexBufferLayout) -> wgpu::RenderPipeline{
    return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[buffer_layout],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState {
                format: texture_format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: create_primitive_type(wgpu::PrimitiveTopology::TriangleList, wgpu::Face::Back),
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    });
}

fn create_primitive_type(p_topology: wgpu::PrimitiveTopology,  cull: wgpu::Face) -> wgpu::PrimitiveState {
    return wgpu::PrimitiveState {
        topology: p_topology,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(cull),
        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
        polygon_mode: wgpu::PolygonMode::Fill,
        // Requires Features::DEPTH_CLAMPING
        clamp_depth: false,
        // Requires Features::CONSERVATIVE_RASTERIZATION
        conservative: false,
    }
}

