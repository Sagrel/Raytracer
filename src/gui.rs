#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::{
    sync::{Arc, Mutex},
    thread,
};

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{CursorGrabMode, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use crate::{
    bvh::Bvh, camera::Camera, config::Config, debug_gui::DebugUi, raytrace::raytrace_in_place,
    scene::Scene, Matrix, Real, Vector,
};

pub struct CameraState {
    pub pitch: Real,
    pub yaw: Real,
    pub position: Vector,
    pub fov: Real,
    pub size: PhysicalSize<u32>,
}

impl CameraState {
    fn get_rotator(&self) -> Matrix {
        Matrix::from_euler(
            glam::EulerRot::default(),
            self.yaw.to_radians(),
            self.pitch.to_radians(),
            Real::to_radians(180.0),
        )
    }

    pub fn move_forward(&mut self, amount: Real) {
        self.position += self.get_rotator().mul_vec3(Vector::Z * amount);
    }
    pub fn move_sideways(&mut self, amount: Real) {
        self.position += self.get_rotator().mul_vec3(Vector::NEG_X * amount);
    }
    pub fn move_vertical(&mut self, amount: Real) {
        self.position += self.get_rotator().mul_vec3(Vector::NEG_Y * amount);
    }

    pub fn build_camera(&self) -> Camera {
        Camera::new_looking_at(
            self.position,
            self.position + self.get_rotator().mul_vec3(Vector::Z),
            self.fov,
            self.size.width as Real / self.size.height as Real,
        )
    }
}


pub(crate) struct UiState {
    pub pixels: Pixels,
    pub debug_ui: DebugUi,
    // Keep the dimensions in a enum to indicate that it has been modified?
    pub camera: CameraState,
    pub reload: bool,
}

fn worker_thread(state: Arc<Mutex<UiState>>, mut config: Config) {
    let mut samples = 0;
    let mut image = Vec::new();
    let scene = Scene::read_scene(&config.scene);
    let bvh = Bvh::new(&scene.shapes);

    loop {
        let camera = {
            let mut state = state.lock().unwrap();
            // Check if the state has changed
            // TODO this is not the best way of doing it probably...
            if state.reload {
                // TODO Modifying the config feels kind of dirty tbh
                let size = state.camera.size;
                config.width = size.width as usize;
                config.height = size.height as usize;
                let num_pixels = (size.width * size.height) as usize;
                // TODO Should this resizing be done in the UI thread to avoid visual artifacts?
                state.pixels.resize_buffer(size.width, size.height).unwrap();
                state
                    .pixels
                    .resize_surface(size.width, size.height)
                    .unwrap();
                state.debug_ui.resize(size.width, size.height);

                if image.len() != num_pixels {
                    image = vec![Vector::default(); num_pixels];
                } else {
                    for pixel in image.iter_mut() {
                        *pixel = Vector::default()
                    }
                }
                samples = 0;
                state.reload = false;
            } else {
                // Display whatever we have already
                render_to_buffer(state.pixels.frame_mut(), &image, samples);
            }
            state.camera.build_camera()
        };
        // Raytrace
        raytrace_in_place(&mut image[..], &config, &scene, &camera, &bvh);
        samples += 1;
    }
}

fn render_to_buffer(screen: &mut [u8], image: &[Vector], samples: usize) {
    for (image_pixel, screen_pixel) in image.iter().zip(screen.chunks_exact_mut(4)) {
        let corrected_color = *image_pixel / samples as Real * 255.0;
        screen_pixel.copy_from_slice(&[
            corrected_color.x as u8,
            corrected_color.y as u8,
            corrected_color.z as u8,
            u8::MAX,
        ])
    }
}

pub fn gui_mode(config: Config) -> Result<(), Error> {
    // TODO include EGUI support for debuggin and stuff
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let size = PhysicalSize::new(config.width as u32, config.height as u32);
    let window = {
        WindowBuilder::new()
            .with_title("Simple raytracer")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    // NOTE: This does not work in WSL...
    window.set_cursor_grab(CursorGrabMode::Confined).unwrap();
    window.set_cursor_visible(false);

    let state = {
        let pixels = Pixels::new(
            size.width,
            size.height,
            SurfaceTexture::new(size.width, size.height, &window),
        )?;

        let debug_ui = DebugUi::new(
            &event_loop,
            size.width,
            size.height,
            window.scale_factor() as f32,
            &pixels,
        );

        Arc::new(Mutex::new(UiState {
            pixels,
            debug_ui,
            reload: true,
            camera: CameraState {
                pitch: 0.0,
                yaw: 0.0,
                position: Vector::new(13.0, 2.0, 3.0),
                fov: 30.0,
                size,
            },
        }))
    };

    let state_clone = state.clone();
    let mut mouse_enabled = false;

    thread::spawn(move || worker_thread(state_clone, config));

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            // Close events
            if input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                state.lock().unwrap().debug_ui.scale_factor(scale_factor);
            }

            // Resize the window
            // Resize event
            if let Some(size) = input.window_resized() {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.camera.size = size;
            }

            // Keyboard events
            if input.key_pressed(VirtualKeyCode::Escape) {
                mouse_enabled = !mouse_enabled;
                window.set_cursor_visible(mouse_enabled);
                window
                    .set_cursor_grab(if mouse_enabled {
                        CursorGrabMode::None
                    } else {
                        CursorGrabMode::Confined
                    })
                    .unwrap();
            }

            // TODO handle keyboard movement?
            if input.key_pressed(VirtualKeyCode::Q) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.camera.fov -= 5.0;
            }
            if input.key_pressed(VirtualKeyCode::E) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.camera.fov += 5.0;
            }
            if input.key_pressed(VirtualKeyCode::W) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.camera.move_forward(0.5);
            }
            if input.key_pressed(VirtualKeyCode::S) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.camera.move_forward(-0.5);
            }
            if input.key_pressed(VirtualKeyCode::A) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.camera.move_sideways(-0.5);
            }
            if input.key_pressed(VirtualKeyCode::D) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.camera.move_sideways(0.5);
            }
            if input.key_pressed(VirtualKeyCode::Space) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.camera.move_vertical(0.5);
            }
            if input.key_pressed(VirtualKeyCode::LShift) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.camera.move_vertical(-0.5);
            }
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                state.lock().unwrap().debug_ui.handle_event(&event);
            }
            Event::RedrawRequested(_) => {
                let mut state = state.lock().unwrap();
                let UiState {
                    pixels,
                    debug_ui,
                    camera,
                    reload,
                    ..
                } = &mut *state;

                // Prepare egui
                debug_ui.prepare(&window, camera, reload);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    debug_ui.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if render_result.is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::DeviceEvent {
                device_id: _,
                event:
                    DeviceEvent::MouseMotion {
                        delta: (x_diff, y_diff),
                    },
            } if !mouse_enabled && (x_diff.abs() > f64::EPSILON || y_diff.abs() > f64::EPSILON) => {
                if x_diff.abs() > f64::EPSILON || y_diff.abs() > f64::EPSILON {
                    let mut state = state.lock().unwrap();

                    state.camera.pitch += y_diff as Real / 5.0;
                    state.camera.yaw += x_diff as Real / 5.0;
                    state.reload = true;
                }
            }
            _ => (),
        }

        window.request_redraw();
    });
}
