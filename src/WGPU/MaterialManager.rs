use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use wgpu::Device;

#[derive(Debug)]
pub struct  MaterialManager {
    material_table : HashMap<String, Box<Material>>,
}

#[derive(Debug)]
pub struct Material {
    pub shader_mudule : wgpu::ShaderModule,
}

impl MaterialManager {
    pub fn new() -> Self {
        return MaterialManager {
            material_table : HashMap::new(),
        }
    }

    pub fn load_shader(&mut self, wgsl_file_path: &String, device : &Device) -> Option<&Box<Material>> {
        let has_value = self.material_table.contains_key(wgsl_file_path);

        if has_value {
            return self.material_table.get(&wgsl_file_path as &str);
        }

        println!("{}", wgsl_file_path);

        let contents = fs::read_to_string(&wgsl_file_path)
            .expect("Something went wrong reading the file");

        // let contents = include_str!("../shader.wgsl");

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some(wgsl_file_path),
            source: wgpu::ShaderSource::Wgsl(contents.into()),
        });

        let created_material = Box::new(Material{
            shader_mudule: shader
        });

        self.material_table.insert(wgsl_file_path.clone(), created_material);

        return self.material_table.get(&wgsl_file_path as &str);
    }
}

