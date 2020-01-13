#![allow(unsafe_code)]

//! Shaders

use crate::console::log;
use glow::HasContext;

#[derive(Copy, Clone, Debug)]
/// Represents a GLSL shader
pub struct Shader {
    /// an id
    pub id: u32,
}

impl Shader {
    /// Creates a new Shader
    pub fn new(gl: &glow::Context, vertex_code: &str, fragment_code: &str) -> Self {
        let shader;

        unsafe {
            // vertex
            let vertex = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(vertex, &vertex_code);
            gl.compile_shader(vertex);
            if !gl.get_shader_compile_status(vertex) {
                log(&vertex_code);
                log(&gl.get_shader_info_log(vertex));
                panic!();
            }

            // fragment
            let fragment = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(fragment, &fragment_code);
            gl.compile_shader(fragment);
            if !gl.get_shader_compile_status(fragment) {
                log(&fragment_code);
                log(&gl.get_shader_info_log(fragment));
                panic!();
            }

            let id = gl.create_program().unwrap();
            gl.attach_shader(id, vertex);
            gl.attach_shader(id, fragment);
            gl.link_program(id);
            if !gl.get_program_link_status(id) {
                log(&gl.get_program_info_log(id));
                panic!();
            }

            shader = Shader { id }
        }

        shader
    }
}
