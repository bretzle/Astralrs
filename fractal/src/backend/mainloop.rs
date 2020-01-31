//! This module contains the main loop

#![allow(unsafe_code)]

use crate::backend::framebuffer::Framebuffer;
use crate::console::Console;
use crate::fractal::Fractal;
use crate::GameState;
use glow::HasContext;
use glutin::{event::Event, event::WindowEvent, event_loop::ControlFlow};
use std::time::Instant;

const TICK_TYPE: ControlFlow = ControlFlow::Poll;

pub fn main_loop<GS: GameState>(mut fractal: Fractal, mut gamestate: GS) {
    unsafe {
        fractal.backend.platform.gl.viewport(
            0,
            0,
            fractal.width_pixels as i32,
            fractal.height_pixels as i32,
        );
    }
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    // We're doing a little dance here to get around lifetime/borrow checking.
    // Removing the context data from FRACTAL in an atomic swap, so it isn't borrowed after move.
    let wrap = std::mem::replace(&mut fractal.backend.platform.context_wrapper, None);
    let unwrap = wrap.unwrap();

    let el = unwrap.el;
    let wc = unwrap.wc;

    el.run(move |event, _, control_flow| {
        *control_flow = TICK_TYPE;

        if fractal.quitting {
            *control_flow = ControlFlow::Exit;
        }

        match event {
            Event::NewEvents(_) => {
                fractal.left_click = false;
                fractal.key = None;
                fractal.shift = false;
                fractal.control = false;
                fractal.alt = false;
            }
            Event::MainEventsCleared => {
                tock(
                    &mut fractal,
                    &mut gamestate,
                    &mut frames,
                    &mut prev_seconds,
                    &mut prev_ms,
                    &now,
                );
                wc.swap_buffers().unwrap();
            }
            Event::LoopDestroyed => (),
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    // Commenting out to see if it helps the Linux world
                    //let dpi_factor = wc.window().hidpi_factor();
                    wc.resize(*physical_size);
                    fractal.resize_pixels(physical_size.width, physical_size.height);
                    unsafe {
                        fractal.backend.platform.gl.viewport(
                            0,
                            0,
                            physical_size.width as i32,
                            physical_size.height as i32,
                        );
                    }
                    fractal.backend.platform.backing_buffer = Framebuffer::build_fbo(
                        &fractal.backend.platform.gl,
                        physical_size.width as i32,
                        physical_size.height as i32,
                    );
                }
                /*WindowEvent::RedrawRequested => {
                    //tock(&mut fractal, &mut gamestate, &mut frames, &mut prev_seconds, &mut prev_ms, &now);
                    wc.swap_buffers().unwrap();
                }*/
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::CursorMoved { position: pos, .. } => {
                    fractal.mouse_pos = (pos.x, pos.y);
                }

                WindowEvent::MouseInput { .. } => {
                    fractal.left_click = true;
                }

                WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput {
                            virtual_keycode: Some(virtual_keycode),
                            state: glutin::event::ElementState::Pressed,
                            modifiers,
                            ..
                        },
                    ..
                } => {
                    fractal.key = Some(*virtual_keycode);
                    if modifiers.shift() {
                        fractal.shift = true;
                    }
                    if modifiers.alt() {
                        fractal.alt = true;
                    }
                    if modifiers.ctrl() {
                        fractal.control = true;
                    }
                }

                _ => (),
            },
            _ => (),
        }
    });
}

/// Internal handling of the main loop.
fn tock<GS: GameState>(
    fractal: &mut Fractal,
    gamestate: &mut GS,
    frames: &mut i32,
    prev_seconds: &mut u64,
    prev_ms: &mut u128,
    now: &Instant,
) {
    let now_seconds = now.elapsed().as_secs();
    *frames += 1;

    if now_seconds > *prev_seconds {
        fractal.fps = *frames as f32 / (now_seconds - *prev_seconds) as f32;
        *frames = 0;
        *prev_seconds = now_seconds;
    }

    let now_ms = now.elapsed().as_millis();
    if now_ms > *prev_ms {
        fractal.frame_time_ms = (now_ms - *prev_ms) as f32;
        *prev_ms = now_ms;
    }

    gamestate.tick(fractal);

    // Console structure - doesn't really have to be every frame...
    for cons in &mut fractal.consoles {
        cons.console.rebuild_if_dirty(&fractal.backend);
    }

    // Bind to the backing buffer
    if fractal.post_scanlines {
        fractal
            .backend
            .platform
            .backing_buffer
            .bind(&fractal.backend.platform.gl);
    }

    // Clear the screen
    unsafe {
        fractal.backend.platform.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        fractal.backend.platform.gl.clear(glow::COLOR_BUFFER_BIT);
    }

    // Tell each console to draw itself
    for cons in &mut fractal.consoles {
        let font = &fractal.fonts[cons.font_index];
        let shader = &fractal.shaders[cons.shader_index];
        cons.console.gl_draw(font, shader, &fractal.backend);
    }

    if fractal.post_scanlines {
        // Now we return to the primary screen
        fractal
            .backend
            .platform
            .backing_buffer
            .default(&fractal.backend.platform.gl);
        unsafe {
            if fractal.post_scanlines {
                fractal.shaders[3].useProgram(&fractal.backend.platform.gl);
                fractal.shaders[3].setVec3(
                    &fractal.backend.platform.gl,
                    "screenSize",
                    fractal.width_pixels as f32,
                    fractal.height_pixels as f32,
                    0.0,
                );
                fractal.shaders[3].setBool(
                    &fractal.backend.platform.gl,
                    "screenBurn",
                    fractal.post_screenburn,
                );
            } else {
                fractal.shaders[2].useProgram(&fractal.backend.platform.gl);
            }
            fractal
                .backend
                .platform
                .gl
                .bind_vertex_array(Some(fractal.backend.platform.quad_vao));
            fractal.backend.platform.gl.bind_texture(
                glow::TEXTURE_2D,
                Some(fractal.backend.platform.backing_buffer.texture),
            );
            fractal
                .backend
                .platform
                .gl
                .draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}
