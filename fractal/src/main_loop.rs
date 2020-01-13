#![allow(unsafe_code)]

//! Main loop

use crate::fractal::Fractal;
use crate::GameState;
use glow::HasContext;
use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use std::time::Instant;
use crate::graphics::framebuffer::Framebuffer;
use crate::Console;

const TICK_TYPE: ControlFlow = ControlFlow::Poll;

/// The main loop
pub fn main_loop<GS: GameState>(mut frac: Fractal, mut gamestate: GS) {
    unsafe {
        frac.backend.platform.gl.viewport(
            0,
            0,
            frac.width_pixels as i32,
            frac.height_pixels as i32,
        );
    }
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    // We're doing a little dance here to get around lifetime/borrow checking.
    // Removing the context data from Fractal in an atomic swap, so it isn't borrowed after move.
    let wrap = std::mem::replace(&mut frac.backend.platform.context_wrapper, None);
    let unwrap = wrap.unwrap();

    let el = unwrap.el;
    let wc = unwrap.wc;

    el.run(move |event, _, control_flow| {
        *control_flow = TICK_TYPE;

        if frac.quitting {
            *control_flow = ControlFlow::Exit;
        }

        match event {
            Event::NewEvents(_) => {
                frac.left_click = false;
                frac.key = None;
                frac.shift = false;
                frac.control = false;
                frac.alt = false;
            }
            Event::MainEventsCleared => {
                tock(
                    &mut frac,
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
                    wc.resize(*physical_size);
                    frac.resize_pixels(physical_size.width, physical_size.height);
                    unsafe {
                        frac.backend.platform.gl.viewport(
                            0,
                            0,
                            physical_size.width as i32,
                            physical_size.height as i32,
                        );
                    }
                    frac.backend.platform.backing_buffer = Framebuffer::build_fbo(
                        &frac.backend.platform.gl,
                        physical_size.width as i32,
                        physical_size.height as i32,
                    );
                }
                /*WindowEvent::RedrawRequested => {
                    //tock(&mut frac, &mut gamestate, &mut frames, &mut prev_seconds, &mut prev_ms, &now);
                    wc.swap_buffers().unwrap();
                }*/
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::CursorMoved { position: pos, .. } => {
                    frac.mouse_pos = (pos.x, pos.y);
                }

                WindowEvent::MouseInput { .. } => {
                    frac.left_click = true;
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
                    frac.key = Some(*virtual_keycode);
                    if modifiers.shift() {
                        frac.shift = true;
                    }
                    if modifiers.alt() {
                        frac.alt = true;
                    }
                    if modifiers.ctrl() {
                        frac.control = true;
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
    frac: &mut Fractal,
    gamestate: &mut GS,
    frames: &mut i32,
    prev_seconds: &mut u64,
    prev_ms: &mut u128,
    now: &Instant,
) {
    let now_seconds = now.elapsed().as_secs();
    *frames += 1;

    if now_seconds > *prev_seconds {
        frac.fps = *frames as f32 / (now_seconds - *prev_seconds) as f32;
        *frames = 0;
        *prev_seconds = now_seconds;
    }

    let now_ms = now.elapsed().as_millis();
    if now_ms > *prev_ms {
        frac.frame_time_ms = (now_ms - *prev_ms) as f32;
        *prev_ms = now_ms;
    }

    gamestate.tick(frac);

    // Console structure - doesn't really have to be every frame...
    for cons in &mut frac.consoles {
        cons.console.rebuild_if_dirty(&frac.backend);
    }

    // Bind to the backing buffer
    if frac.post_scanlines {
        frac.backend
            .platform
            .backing_buffer
            .bind(&frac.backend.platform.gl);
    }

    // Clear the screen
    unsafe {
        frac.backend.platform.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        frac.backend.platform.gl.clear(glow::COLOR_BUFFER_BIT);
    }

    // Tell each console to draw itself
    for cons in &mut frac.consoles {
        let font = &frac.fonts[cons.font_index];
        let shader = &frac.shaders[cons.shader_index];
        cons.console.draw(font, shader, &frac.backend);
    }

    if frac.post_scanlines {
        // Now we return to the primary screen
        frac.backend
            .platform
            .backing_buffer
            .default(&frac.backend.platform.gl);
        unsafe {
            if frac.post_scanlines {
                frac.shaders[3].use_program(&frac.backend.platform.gl);
                frac.shaders[3].set_vec3(
                    &frac.backend.platform.gl,
                    "screenSize",
                    frac.width_pixels as f32,
                    frac.height_pixels as f32,
                    0.0,
                );
                frac.shaders[3].set_bool(
                    &frac.backend.platform.gl,
                    "screenBurn",
                    frac.post_screenburn,
                );
            } else {
                frac.shaders[2].use_program(&frac.backend.platform.gl);
            }
            frac.backend
                .platform
                .gl
                .bind_vertex_array(Some(frac.backend.platform.quad_vao));
            frac.backend.platform.gl.bind_texture(
                glow::TEXTURE_2D,
                Some(frac.backend.platform.backing_buffer.texture),
            );
            frac.backend.platform.gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}
