#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::{
    sync::{Arc, Mutex},
    thread,
};

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{DeviceEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{
    bvh::Bvh, camera::Camera, config::Config, raytrace::raytrace_in_place, scene::Scene, Real,
    Vector,
};

struct UiState {
    pub pixels: Pixels,
    // Keep the dimensions in a enum to indicate that it has been modified?
    pub width: usize,
    pub height: usize,
    pub pitch: Real,
    pub pos: Vector,
    pub fov: Real,
    pub yaw: Real, // TODO Add more fields to customize the experience
    pub reload: bool,
}

fn worker_thread(state: Arc<Mutex<UiState>>, config: Config) {
    let mut samples = 0;
    let mut image = Vec::new();
    let scene = Scene::read_scene(&config.scene);
    let bvh = Bvh::new(&scene.shapes);

    loop {
        let camera = {
            let mut state = state.lock().unwrap();

            // Display whatever we have already
            render_to_buffer(state.pixels.frame_mut(), &image, samples);
            state.pixels.render().unwrap();

            // Check if the state has changed
            if state.reload {
                image = vec![Vector::default(); state.width * state.height];
                samples = 0;
                state.reload = false;
            }
            Camera::new_angles(
                state.fov,
                state.pitch,
                state.yaw,
                state.pos,
                state.width as Real / state.height as Real,
            )
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

    let window = {
        let size = LogicalSize::new(config.width as f64, config.height as f64);
        let scaled_size = LogicalSize::new(config.width as f64, config.height as f64);
        WindowBuilder::new()
            .with_title("Simple raytracer")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // TODO This does not work, is it a WSL thing? Yes, yest it is. It works fine in windows
    window
        .set_cursor_grab(winit::window::CursorGrabMode::Confined)
        .unwrap();
    window.set_cursor_visible(false);

    let state = {
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(config.width as u32, config.height as u32, surface_texture)?
        };
        Arc::new(Mutex::new(UiState {
            pixels,
            width: config.width,
            height: config.height,
            pitch: 0.0,
            yaw: 0.0,
            fov: 20.0,
            reload: true,
            pos: Vector::new(13.0, 2.0, 3.0),
        }))
    };

    let state_clone = state.clone();

    thread::spawn(move || worker_thread(state_clone, config));

    event_loop.run(move |event, _, control_flow| {
        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.

        if let winit::event::Event::DeviceEvent {
            device_id: _,
            event: DeviceEvent::MouseMotion {
                delta: (x_diff, y_diff),
            },
        } = event
        {
            if x_diff.abs() > f64::EPSILON || y_diff.abs() > f64::EPSILON {
                let mut state = state.lock().unwrap();

                state.pitch += y_diff as Real / 5.0;
                state.yaw += x_diff as Real / 5.0;
                println!("pitch {}º Yaw {}º", state.pitch, state.yaw);
                state.reload = true;
            }
        }

        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // TODO handle keyboard movement?
            if input.key_pressed(VirtualKeyCode::Q) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.fov -= 5.0;
            }
            if input.key_pressed(VirtualKeyCode::E) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.fov += 5.0;
            }
            if input.key_pressed(VirtualKeyCode::W) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.pos.x += 0.5;
            }
            if input.key_pressed(VirtualKeyCode::E) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.fov += 5.0;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.width = size.width as usize;
                state.height = size.height as usize;
                if state
                    .pixels
                    .resize_surface(size.width, size.height)
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }
            }
        }
    });
}
