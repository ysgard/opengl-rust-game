extern crate gl;
extern crate sdl2;

pub mod render_gl;
pub mod resources;

use std::ffi::{CString, CStr};
use self::resources::Resources;
use std::path::Path;

fn main() {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    // Set minimal version of OpenGL to use
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let qgl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vert_shader = render_gl::Shader::from_vert_source(
        &gl, &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(
        &gl, &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    let shader_program = render_gl::Program::from_res(
        &gl, &res, "shaders/triangle"
    ).unwrap();

    shader_program.set_used();

    let vertices: Vec<f32> = vec![
        // positions      //colors
        -0.75, -0.5, 0.0,  1.0, 0.0, 0.0, //bottom right
        0.25, -0.5, 0.0,   0.0, 1.0, 0.0, // bottom left
        -0.25, 0.5, 0.0,    0.0, 0.0, 1.0,  // top
        0.75, 0.5, 0.0,    1.0, 0.0, 0.0   // top right
    ];
    let indices: Vec<u32> = vec![
        0, 1, 2, // first triangle
        1, 2, 3, // second triangle
    ];

    let mut vbo: gl::types::GLuint = 0;  // Vertex Buffer Object (verices)
    let mut ebo: gl::types::GLuint = 0;  // Element Buffer Object (indices)
    unsafe {
        gl.GenBuffers(1, &mut vbo);
        gl.GenBuffers(1, &mut ebo);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo); // Bind the vbo buffer to ARRAY_BUFFER target
        gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo); // Bind the ebo to ELEMENT_ARRAY_BUFFER
        gl.BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl.BufferData(
            gl::ELEMENT_ARRAY_BUFFER, // target
            (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, // size of data in bytes
            indices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffers
        gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo); // Rebinding the vbo is wasteful here, done to show need for vbo

        gl.EnableVertexAttribArray(0); //layout (location = 0) in vertex shader_program
        gl.VertexAttribPointer(
            0, // index of the position vertex attribute
            3, // Number of components per position vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // don't normalize (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride
            std::ptr::null() // offset of first component
        );

        gl.EnableVertexAttribArray(1); //layout (location = 1) in vertex shader program
        gl.VertexAttribPointer(
            1, // index of the color vertex attribute
            3, // number of components per color vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // not normalized
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );

        // unbind both vbo and vba
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    println!("size of Gl: {}", std::mem::size_of_val(&gl));

    // Wireframe mode
    // unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw the triangles
        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(vao);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl.DrawElements(
                gl::TRIANGLES, // mode
                6, // number of elements we want to draw (6 indices, so 6 elements)
                gl::UNSIGNED_INT, // Type of the indices
                std::ptr::null() // offset into the indices
            );
        }

        window.gl_swap_window();
    }
}
