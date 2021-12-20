use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use serde::de::Unexpected::Option;
use serde_json::json;
use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;
use crate::{ObjectDataDefineJSON, WGPUConstructor};
use crate::Type::ObjectBufferType::{ObjectBufferJSON, Vertex};

pub struct VertexBuffer {
    pub label: String,
    pub number_of_vertices : u32,
    pub buffer_data: Buffer,
    pub raw_data: Vec<Vertex>,
}

impl VertexBuffer {
    pub fn new(label:String, data: Vec<Vertex>, buffer_data: Buffer) -> VertexBuffer{
        VertexBuffer::create_struct(
            label, data, buffer_data
        )
    }

    pub fn set_buffer(&mut self, data: Vec<Vertex>, buffer_data: Buffer) {
        self.number_of_vertices= (data.len() as u32);
        self.raw_data = data;
        self.buffer_data = buffer_data;
    }

    fn create_struct(label:String, data: Vec<Vertex>, buffer_data: Buffer) -> VertexBuffer{
        VertexBuffer {
            label: label,
            number_of_vertices: data.len() as u32,
            raw_data: data,
            buffer_data: buffer_data,
        }
    }
}

pub struct VertexBufferManager {
    device: Rc<Device>,
    vertex_buffer_list: HashMap<String, RefCell<VertexBuffer>>,
}

impl VertexBufferManager {

    pub fn new(device: Rc<Device>) -> Self {
        Self {
            device: device,
            vertex_buffer_list: HashMap::new(),
        }
    }

    pub fn get_all_vertex_buffer(&self) ->  Vec<&RefCell<VertexBuffer>>{
        return self.vertex_buffer_list.keys().map(
            move |x| self.vertex_buffer_list.get(x).unwrap()
        ).collect();
    }

    pub fn create_edit_vertex(&mut self, id: String, vertex_list: Vec<Vertex>) {
        let buffer = self.create_buffer(&vertex_list);
        let buffer_list_option = self.vertex_buffer_list.get_mut(&id);

        match buffer_list_option {
            Some(x) =>{
                x.get_mut().set_buffer(vertex_list, buffer);
            },
            None => {
                self.create_vertex_buffer(&id, vertex_list);
            },
        }
    }

    pub fn insert_json_data(&mut self, objectDefineJSON : &ObjectDataDefineJSON) {
        let mut i_obj_vertex_array: Vec<Vec<Vertex>> = Vec::new();

        for x in &objectDefineJSON.objects {
            self.create_vertex_buffer_from_jsondata(&x);
        }
    }

    fn create_vertex_buffer_from_jsondata(&mut self, jsondata: &ObjectBufferJSON) {
        let mut vertex_array: Vec<Vertex> = Vec::new();
        let color = crate::UtilityFunc::parse_rawstring_to_vector3(&jsondata.color);

        for vertexString in &jsondata.vertex {
            let position = crate::UtilityFunc::parse_rawstring_to_vector3(&vertexString);

            vertex_array.push(Vertex{
                color: color.clone(),
                position : position,
            });
        }

        self.create_edit_vertex(jsondata.name.clone(), vertex_array);
    }

    fn create_vertex_buffer(&mut self, id: &String, vertex_list: Vec<Vertex>)
    {
        let buffer = self.create_buffer(&vertex_list);
        let vertex_buffer = VertexBuffer::new(id.clone(), vertex_list, buffer);

        self.vertex_buffer_list.insert(id.clone(), RefCell::new(vertex_buffer));
    }

    fn create_buffer(&mut self, vertex_list: &Vec<Vertex>) -> Buffer {
        return self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertex_list),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
    }
}
