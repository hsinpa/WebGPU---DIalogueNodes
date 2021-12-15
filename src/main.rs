mod GPUState;
mod WGPU;
mod Type;
mod Utility;

use GPUState::State as State;
use WGPU::WGPUComponent::WGPUConstructor as WGPUConstructor;
use WGPU::WGPUManager::WGPUManager as WGPUManager;
use WGPU::MaterialManager;
use WGPU::RenderPipelineUtility;
use Utility::UtilityFunc;
use Type::ObjectBufferType::ObjectDataDefineJSON;
use std::iter;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // State::new uses async code, so we're going to wait for it to finish
    let mut wgpu_construtor : WGPUConstructor = (WGPUConstructor::new(&window));
    let mut wgpu_manager : WGPUManager = (WGPUManager::new(wgpu_construtor));

    let filePath = String::from("./assets/data/objects_data.json");
    let object_data_json = UtilityFunc::parse_json_file::<ObjectDataDefineJSON>(&filePath);

    match object_data_json {
        Some(x) => println!("{}", x.objects.len()),
        None => println!("Nothing is here")
    }


    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },

                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                wgpu_manager.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &&mut so w have to dereference it twice
                wgpu_manager.resize(**new_inner_size);
            },
            _ => {}
        },

        Event::RedrawRequested(_) => {
            match wgpu_manager.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => wgpu_manager.resize(window.inner_size()),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        },
        Event::RedrawEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        },
        _ => {}
    });


    // let mut state: State = pollster::block_on(State::new(&window));
    //
    // event_loop.run(move |event, _, control_flow| {
    //     match event {
    //         Event::WindowEvent {
    //             ref event,
    //             window_id,
    //         } if window_id == window.id() => {
    //             if !state.input(event) {
    //                 // UPDATED!
    //                 match event {
    //                     WindowEvent::CloseRequested
    //                     | WindowEvent::KeyboardInput {
    //                         input:
    //                         KeyboardInput {
    //                             state: ElementState::Pressed,
    //                             virtual_keycode: Some(VirtualKeyCode::Escape),
    //                             ..
    //                         },
    //                         ..
    //                     } => *control_flow = ControlFlow::Exit,
    //                     WindowEvent::Resized(physical_size) => {
    //                         state.resize(*physical_size);
    //                     }
    //                     WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
    //                         // new_inner_size is &&mut so w have to dereference it twice
    //                         state.resize(**new_inner_size);
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //         Event::RedrawRequested(_) => {
    //             state.update();
    //             match state.render() {
    //                 Ok(_) => {}
    //                 // Reconfigure the surface if lost
    //                 Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
    //                 // The system is out of memory, we should probably quit
    //                 Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
    //                 // All other errors (Outdated, Timeout) should be resolved by the next frame
    //                 Err(e) => eprintln!("{:?}", e),
    //             }
    //         }
    //         Event::RedrawEventsCleared => {
    //             // RedrawRequested will only trigger once, unless we manually
    //             // request it.
    //             window.request_redraw();
    //         }
    //         _ => {}
    //     }
    //    });
}
