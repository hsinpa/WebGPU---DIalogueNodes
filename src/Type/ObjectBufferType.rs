use serde_json::{Deserializer, Serializer};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ObjectBufferJSON {
    pub name : String,
    pub position_x : i8,
    pub position_y : i8,
    pub color : String,
    pub size : i8,
    pub vertex : Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ObjectDataDefineJSON {
    pub objects : Vec<ObjectBufferJSON>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct VertexTriangleIndex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}