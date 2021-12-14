use std::future::Future;
use std::iter;
use wgpu::{Adapter, Device, Instance, Queue, Surface};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit::dpi::PhysicalSize;

pub struct WGPUConstructor {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

}

impl WGPUConstructor {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = WGPUConstructor::create_surface(&instance, window);

        let adapter =pollster::block_on(WGPUConstructor::create_adapter(&instance, &surface));
        let (device, queue) = pollster::block_on(WGPUConstructor::create_device_and_queue(&adapter));

        let surface_config = WGPUConstructor::create_surface_config(&surface,&adapter,&size);
            surface.configure(&device, &surface_config);

        return Self {
            surface: surface,
            device: device,
            queue: queue,
            config: surface_config,
            size: size,
        };
    }

    fn create_surface(instance: &Instance, window: &Window) -> Surface {
        return unsafe { instance.create_surface(window) };
    }

    async fn create_adapter(instance: &Instance, surface : &Surface) -> Adapter {
        return instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }).await.unwrap();
    }

    async fn create_device_and_queue(adapter : &Adapter) -> (Device, Queue){
        return adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();
    }

    fn create_surface_config(surface : &Surface, adapter : &Adapter, size : &PhysicalSize<u32>) -> wgpu::SurfaceConfiguration {
        return wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
    }
}
