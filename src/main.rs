mod GPUState;
mod WGPU;


use GPUState::State as State;
use WGPU::WGPUConstructor::WGPUConstructor as WGPUConstructor;
use WGPU::WGPUManager::WGPUManager as WGPUManager;
use WGPU::MaterialManager;
use WGPU::RenderPipelineManager;

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
            _ => {}
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
