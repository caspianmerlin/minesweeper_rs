#![allow(unused)]

use std::{num::NonZeroU32, ffi::{CStr, CString, c_void}};

use gl::types::{GLfloat, GLsizeiptr, GLsizei};
use glutin::{display::{Display, DisplayApiPreference}, prelude::{GlDisplay, GlConfig, NotCurrentGlContextSurfaceAccessor}, config::{ConfigTemplate, ConfigTemplateBuilder, ConfigSurfaceTypes, Config}, context::{ContextAttributesBuilder, ContextApi, Version, GlProfile, PossiblyCurrentContext}, surface::{Surface, WindowSurface, SurfaceAttributes, SurfaceAttributesBuilder, GlSurface, SwapInterval}};
use raw_window_handle::{HasRawWindowHandle, HasRawDisplayHandle, RawWindowHandle};
use winit::{event_loop::EventLoop, window::{WindowBuilder, Window}, event::{Event, WindowEvent}, dpi::{Size, PhysicalSize}};

use crate::util::{LegacyRandomNumberGenerator, RandomNumberGenerator, ModernRandomNumberGenerator};


mod config;
mod util;
mod grid;
mod graphics;



fn main() {
    let mut config = config::Config::load();

    let event_loop = EventLoop::new();
    let raw_display = event_loop.raw_display_handle();
    let window_size = PhysicalSize::new(300, 300);
    let mut window = Some(WindowBuilder::new().with_transparent(true).with_decorations(true).with_resizable(false).with_title("Minesweeper").with_inner_size(window_size).with_position(config.window_position).build(&event_loop).unwrap());
    let raw_window_handle = window.as_ref().map(|w| w.raw_window_handle());
    let gl_display = unsafe {
        #[cfg(target_os = "windows")]
        let preference = DisplayApiPreference::Wgl(Some(raw_window_handle.unwrap()));
        
        #[cfg(target_os = "linux")]
        let preference = DisplayApiPreference::Glx(Box::new(winit::platform::unix::register_xlib_error_hook));
        Display::new(raw_display, preference).unwrap()
    };
    println!("Running on: {}", gl_display.version_string());
    let template = config_template(raw_window_handle);
    let glutin_config = unsafe { gl_display.find_configs(template) }.unwrap().reduce(|accum, config| {
        if config.num_samples() > accum.num_samples() {
            config
        } else {
            accum
        }
    }).unwrap();
    println!("Picked a config with {} samples", glutin_config.num_samples());
    let context_attributes = ContextAttributesBuilder::new().with_context_api(ContextApi::OpenGl(Some( Version::new(3, 3)))).with_profile(GlProfile::Core).build(raw_window_handle);

    let mut not_current_gl_context = Some(
        unsafe {
            gl_display.create_context(&glutin_config, &context_attributes).expect("Failed to create OpenGL context")
        }
    );

    let mut gl_state = None;


    let vertices: [f32; 75] = [
        // Square
        -0.9, 0.9,      1.0, 0.0, 0.0,  // 0
        -0.9, 0.2,      1.0, 0.0, 0.0,  // 1
        -0.2, 0.9,      1.0, 0.0, 0.0,  // 2
        -0.2, 0.2,      1.0, 0.0, 0.0,  // 3

        // Triangle
        0.4, 0.9,       0.0, 1.0, 0.0,  // 4
        0.6, 0.2,       0.0, 1.0, 0.0,  // 5
        0.2, 0.2,       0.0, 1.0, 0.0,  // 6

        // Rectangle
        0.2, -0.2,      0.0, 0.0, 1.0,  // 7
        0.6, -0.2,      0.0, 0.0, 1.0,  // 8
        0.2, -0.9,      0.0, 0.0, 1.0,  // 9
        0.6, -0.9,      0.0, 0.0, 1.0,  // 10

        // Diamond
        -0.5, -0.2,     1.0, 1.0, 0.0,  // 11
        -0.2, -0.5,     1.0, 1.0, 0.0,  // 12
        -0.8, -0.5,     1.0, 1.0, 0.0,  // 13
        -0.5, -0.8,     1.0, 1.0, 0.0,  // 14
    ];

    let indices = [
        0, 1, 2, 1, 2, 3,
        4, 5, 6,
        7, 8, 9, 8, 9, 10,
        11, 12, 13, 12, 13, 14,
    ];

    let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
    

    let mut shader_program = None;





    event_loop.run(move|event, event_loop_window_target, control_flow| {
        control_flow.set_wait();
        match event {
            Event::Resumed => {
                let window = window.take().unwrap_or_else(|| {
                    println!("Got here.");
                    let window = WindowBuilder::new();
                    window.build(event_loop_window_target).unwrap()
                });
                let gl_window = GlWindow::from_existing(&gl_display, window, &glutin_config);
                let gl_context = not_current_gl_context.take().unwrap().make_current(&gl_window.surface).unwrap();
                if let Err(res) = gl_window.surface.set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap())) {
                    println!("Error setting vSync: {:?}", res);
                }
                gl::load_with(|name| {
                    let name_cstring = CString::new(name).unwrap();
                    let name_cstr = name_cstring.as_c_str();
                    gl_display.get_proc_address(&name_cstr)
                });
                gl_state.replace(GlState { window: gl_window, context: gl_context });


                unsafe {
                    gl::GenVertexArrays(1, &mut vao);
                    gl::GenBuffers(1, &mut vbo);
                    gl::GenBuffers(1, &mut ebo);
            
                    gl::BindVertexArray(vao);
            
                    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                    gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr, &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW);
            
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr, &indices[0] as *const i32 as *const c_void, gl::STATIC_DRAW);
            
                    gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<GLfloat>() as GLsizei, std::ptr::null());
                    gl::EnableVertexAttribArray(0);
            
                    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<GLfloat>() as GLsizei, (2 * std::mem::size_of::<GLfloat>()) as *const c_void);
                    gl::EnableVertexAttribArray(1);
            
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    gl::BindVertexArray(0);
                }
                shader_program = Some(graphics::opengl::shader::ShaderProgram::new(include_str!("graphics/opengl/shaders/vertex.glsl"), include_str!("graphics/opengl/shaders/fragment.glsl")).unwrap());

            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::Moved(new_position) => config.window_position = new_position,
                _ => {},
            }
            Event::RedrawEventsCleared => {
                if let Some(gl_state) = &gl_state {
                    //Render code here
                    unsafe {
                        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                        shader_program.as_ref().unwrap().bind();
                        gl::BindVertexArray(vao);
                        gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
                    }

                    gl_state.window.surface.swap_buffers(&gl_state.context).unwrap();
                }
            },
            Event::LoopDestroyed => {
                config.save_to_ini();
            }
            _ => {},
        }
    });



}

fn config_template(raw_window_handle: Option<RawWindowHandle>) -> ConfigTemplate {
    let mut builder = ConfigTemplateBuilder::new().with_alpha_size(8);
    if let Some(raw_window_handle) = raw_window_handle {
        builder = builder.compatible_with_native_window(raw_window_handle).with_surface_type( ConfigSurfaceTypes::WINDOW );
    }
    builder.build()

}

/// Structure to hold winit window and gl surface.
pub struct GlWindow {
    pub surface: Surface<WindowSurface>,
    pub window: Window,
}

impl GlWindow {
    pub fn from_existing(display: &Display, window: Window, config: &Config) -> Self {
        let attrs = surface_attributes(&window);
        let surface = unsafe { display.create_window_surface(config, &attrs).unwrap() };
        Self { window, surface }
    }
}

/// Create surface attributes for window surface.
pub fn surface_attributes(window: &Window) -> SurfaceAttributes<WindowSurface> {
    let (width, height): (u32, u32) = window.inner_size().into();
    let raw_window_handle = window.raw_window_handle();
    SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
    )
}

struct GlState {
    window: GlWindow,
    context: PossiblyCurrentContext,
}