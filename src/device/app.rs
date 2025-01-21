use std::cell::RefCell;
use log::{info, warn};
use shipyard::{UniqueView, UniqueViewMut, World};
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;
use cgmath::num_traits::signum;
use smaa::{SmaaMode, SmaaTarget};
//use wasm_bindgen::JsCast;

use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration, SurfaceError, SurfaceTexture, TextureFormat};

use crate::device::background_pipleine::BackGroundPipeLine;
use crate::device::camera::{camera_zoom, update_camera_by_mouse, Camera};
use crate::device::graphics::{init_graphics, key_frame, mouse_left_pressed, mouse_move, on_keyboard, render, render_selection, resize_window, set_right_mouse_pressed, uievent, unset_right_mouse_pressed, GlobalState, Graphics};
use crate::device::mesh_pipeline::MeshPipeLine;
use crate::device::txt_pipeline::TxtPipeLine;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize};
use winit::error::OsError;
use winit::event::{DeviceEvent, DeviceId, ElementState, MouseButton, MouseScrollDelta, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowAttributes, WindowId};
use crate::ui::ui_renderer::EguiRenderer;

pub struct App {
    pub world: World,
    pub event_loop_proxy: Arc<EventLoopProxy<Graphics>>,
    pub is_world_up: bool,
}

impl App {
    pub fn new(el: &EventLoop<Graphics>) -> Self {
        let event_loop_proxy: EventLoopProxy<Graphics> = el.create_proxy();
        Self {
            world: World::new(),
            event_loop_proxy: Arc::new(event_loop_proxy),
            is_world_up: false,
        }
    }
}

impl ApplicationHandler<Graphics> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        {
            let proxy = self.event_loop_proxy.clone();
            let gfx: Graphics = pollster::block_on(create_graphics(event_loop));
            assert!(proxy.send_event(gfx).is_ok());
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: Graphics) {
        self.is_world_up = true;
        let w = event.window.clone();
        init_graphics(&self.world, event);
        w.request_redraw();

        //self.world.add_unique(event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if (self.is_world_up) {
            self.world.run_with_data(uievent, &event);
            match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    self.world.run(key_frame);
                    self.world.run(render);
                }
                WindowEvent::ActivationTokenDone { .. } => {}
                WindowEvent::Resized(physical_size) => {
                    self.world.run_with_data(resize_window, physical_size);
                }
                WindowEvent::Moved(_) => {}
                WindowEvent::Destroyed => {}
                WindowEvent::DroppedFile(_) => {}
                WindowEvent::HoveredFile(_) => {}
                WindowEvent::HoveredFileCancelled => {}
                WindowEvent::Focused(_) => {}
                WindowEvent::KeyboardInput {
                    device_id,
                    event,
                    is_synthetic,
                } => {
                    self.world.run_with_data(on_keyboard, event);
                }
                WindowEvent::ModifiersChanged(_) => {}
                WindowEvent::Ime(_) => {}
                WindowEvent::CursorMoved { device_id, position } => {
                    self.world.run_with_data(mouse_move, position);
                }
                WindowEvent::CursorEntered { .. } => {}
                WindowEvent::CursorLeft { .. } => {}
                WindowEvent::MouseWheel { device_id, delta, phase } => {
                    match delta {
                        MouseScrollDelta::LineDelta(h, v) => {
                            self.world.run_with_data(camera_zoom, v);
                        }
                        MouseScrollDelta::PixelDelta(p) => {
                            self.world.run_with_data(camera_zoom, signum(p.y as f32));
                        }
                    }
                }
                WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                } => match button {
                    MouseButton::Left => match state {
                        ElementState::Pressed => {
                            self.world.run(mouse_left_pressed);
                            self.world.run(render_selection);
                            
                        }
                        ElementState::Released => {}
                    },
                    MouseButton::Right => match state {
                        ElementState::Pressed => {
                            self.world.run(set_right_mouse_pressed);
                        }
                        ElementState::Released => {
                            self.world.run(unset_right_mouse_pressed);
                        }
                    },
                    MouseButton::Middle => match state {
                        ElementState::Pressed => {}
                        ElementState::Released => {}
                    },
                    MouseButton::Back => match state {
                        ElementState::Pressed => {}
                        ElementState::Released => {}
                    },
                    MouseButton::Forward => match state {
                        ElementState::Pressed => {}
                        ElementState::Released => {}
                    },
                    MouseButton::Other(_) => {}
                },
                WindowEvent::PinchGesture { .. } => {}
                WindowEvent::PanGesture { .. } => {}
                WindowEvent::DoubleTapGesture { .. } => {}
                WindowEvent::RotationGesture { .. } => {}
                WindowEvent::TouchpadPressure { .. } => {}
                WindowEvent::AxisMotion { .. } => {}
                WindowEvent::Touch(_) => {}
                WindowEvent::ScaleFactorChanged { .. } => {}
                WindowEvent::ThemeChanged(_) => {}
                WindowEvent::Occluded(_) => {}
            }
        }
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent, ) {
        if (self.is_world_up) {
            match event {
                DeviceEvent::Added => {}
                DeviceEvent::Removed => {}
                DeviceEvent::MouseMotion { delta } => {
                    self.world.run_with_data(update_camera_by_mouse, delta);
                }
                DeviceEvent::Motion { .. } => {}
                DeviceEvent::Button { .. } => {}
                DeviceEvent::Key(_) => {}
                DeviceEvent::MouseWheel { .. } => {}
            }
        }
    }
}


fn create_graphics(event_loop: &ActiveEventLoop) -> impl Future<Output = Graphics> + 'static {
    let wsize: PhysicalSize<u32> = winit::dpi::PhysicalSize::new(1024, 768);
    let window_attrs = Window::default_attributes().with_inner_size(wsize.clone());
    let rc_window: Arc<Window> = Arc::new(event_loop.create_window(window_attrs).unwrap());
    async move {
        let (instanse, adapter): (Instance, Adapter) = {
            match create_primary().await {
                None => {
                    panic!("NOT POSSIBLE TO FOUND SUITABLE GPU")
                }
                Some((instanse, adapter)) => (instanse, adapter),
            }
        };

        let (device, queue): (Device, Queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: Default::default(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface: Surface = instanse.create_surface(rc_window.clone()).unwrap();

        let surface_config: SurfaceConfiguration = surface
            .get_default_config(&adapter, wsize.width as u32, wsize.height as u32)
            .unwrap();

        info!("ADAPTER ATTRIBS {:?}", adapter.get_info());
        info!("SURFACE ATTRIBS {:?}", surface_config);
        surface.configure(&device, &surface_config);
        //let window_size: PhysicalSize<u32> = rc_window.inner_size().clone();
        let format: TextureFormat = surface.get_current_texture().unwrap().texture.format();
        let background_pipe_line: BackGroundPipeLine =
            BackGroundPipeLine::new(&device, &format, wsize.width as i32, wsize.height as i32);

        let mesh_pipe_line =
            MeshPipeLine::new(&device, &format, wsize.width as i32, wsize.height as i32);

        let txt_pipe_line = TxtPipeLine::new(
            &device,
            &queue,
            &format,
            wsize.width as i32,
            wsize.height as i32,
        );
        let mut smaa_target = Rc::new( RefCell::new(SmaaTarget::new(
            &device,
            &queue,
            surface_config.width,
            surface_config.height,
            surface_config.format,
            SmaaMode::Smaa1X,
        )));
        Graphics {
            device: device,
            adapter: adapter,
            queue: queue,
            window: rc_window,
            surface: surface,
            surface_config: surface_config,
            smaa_target: smaa_target,
            background_pipe_line: background_pipe_line,
            camera: Camera::default(),
            mesh_pipe_line: mesh_pipe_line,
            txt_pipe_line: txt_pipe_line,

        }
    }
}

async fn create_primary() -> Option<(Instance, Adapter)> {
    let inst_descr=wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        flags: Default::default(),
        backend_options: Default::default(),
    };

    let instance: Instance = wgpu::Instance::new(&inst_descr);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: None, // Some(&surface)
            power_preference: wgpu::PowerPreference::None,
            force_fallback_adapter: false,
        })
        .await;
    match adapter {
        None => None,
        Some(adapt) => Some((instance, adapt)),
    }
}


