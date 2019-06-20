#[cfg(debug_assertions)]
macro_rules! glcheck {
    ($gl_expr:expr) => {
        {
            // If there was an error here before we ran our expression, someone was bad and didn't
            // check their error.
            if gl::GetError() != gl::NO_ERROR {
                log::error!("Unchecked OpenGL error!");
            }
            let result = $gl_expr;
            if gl::GetError() != gl::NO_ERROR {
                log::error!("OpenGL error: {}:{} -> {}", file!(), line!(), stringify!($gl_expr));
            }
            result
        }
    };
}

#[cfg(not(debug_assertions))]
macro_rules! glcheck {
    ($gl_expr:expr) => { $gl_expr };
}

mod event;
mod gfx;

use std::ffi::CString;

use gl::types::{GLuint, GLsizei, GLint, GLsizeiptr};

use tcrab_console::canvas::GlyphLibrary;

const VERTEX_SHADER_SRC: &[u8] = include_bytes!("../shaders/vertex.glsl");
const FRAGMENT_SHADER_SRC: &[u8] = include_bytes!("../shaders/fragment.glsl");

#[derive(Debug)]
pub enum CreationError {
    Glutin(glutin::CreationError),
    Context(glutin::ContextError),
}

impl std::fmt::Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CreationError::Glutin(err) => write!(f, "{}", err),
            CreationError::Context(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for CreationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub title: String,
    pub texture_width: u32,
    pub texture_height: u32,
    pub texture_data: Vec<u8>,
    pub cell_width: u32,
    pub cell_height: u32,
}

pub struct Console {
    events_loop: glutin::EventsLoop,
    windowed_context: glutin::WindowedContext<glutin::PossiblyCurrent>,
    shader_program: GLuint,
    projection_uniform_location: GLint,
    texture_uniform_location: GLint,
    vertex_array_object: GLuint,
    vertex_buffer_object: GLuint,
    index_buffer_object: GLuint,
    texture: GLuint,
    texture_width: u32,
    texture_height: u32,
    cell_width: u32,
    cell_height: u32,
}

impl Console {
    pub fn new(settings: Settings) -> Result<Console, CreationError> {
        let events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_visibility(false)
            .with_resizable(false)
            .with_title(settings.title);
        let windowed_context = glutin::ContextBuilder::new()
            .build_windowed(window_builder, &events_loop)
            .map_err(CreationError::Glutin)?;
        let windowed_context = unsafe {
            windowed_context
                .make_current()
                .map_err(|(_, err)| CreationError::Context(err))?
        };
        gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

        let shader_program = unsafe {
            gfx::link_program(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC)
        };

        let projection_uniform_location = unsafe {
            let name = CString::new("u_Projection").unwrap(); // Should never fail
            glcheck!(gl::GetUniformLocation(shader_program, name.as_ptr()))
        };

        let texture_uniform_location = unsafe {
            let name = CString::new("u_Texture").unwrap(); // Should never fail
            glcheck!(gl::GetUniformLocation(shader_program, name.as_ptr()))
        };

        let vertex_array_object = unsafe {
            let mut id = 0;
            glcheck!(gl::GenVertexArrays(1, &mut id));
            id
        };

        let (vertex_buffer_object, index_buffer_object) = unsafe {
            let mut ids = [0, 0];
            glcheck!(gl::GenBuffers(ids.len() as GLsizei, ids.as_mut_ptr()));
            (ids[0], ids[1])
        };

        unsafe {
            gfx::Vertex::setup_vertex_array_object(vertex_array_object, vertex_buffer_object);
        }

        let texture = unsafe {
            let mut id = 0;
            glcheck!(gl::GenTextures(1, &mut id));
            glcheck!(gl::BindTexture(gl::TEXTURE_2D, id));
            glcheck!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32));
            glcheck!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32));
            glcheck!(gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                settings.texture_width as GLsizei,
                settings.texture_height as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                settings.texture_data.as_ptr() as *const _,
            ));
            id
        };

        Ok(Console {
            events_loop,
            windowed_context,
            shader_program,
            projection_uniform_location,
            texture_uniform_location,
            vertex_array_object,
            vertex_buffer_object,
            index_buffer_object,
            texture,
            texture_width: settings.texture_width,
            texture_height: settings.texture_height,
            cell_width: settings.cell_width,
            cell_height: settings.cell_height,
        })
    }
}

impl tcrab_console::Console for Console {
    type GlyphDef = TextureRegion;

    fn wait_for_events_forever<F>(&mut self, mut event_handler: F)
    where
        F: FnMut(tcrab_console::Event) -> tcrab_console::ControlFlow,
    {
        self.events_loop.run_forever(|glutin_event| {
            if let Some(event) = event::translate(glutin_event) {
                match event_handler(event) {
                    tcrab_console::ControlFlow::Continue => glutin::ControlFlow::Continue,
                    tcrab_console::ControlFlow::Break => glutin::ControlFlow::Break,
                }
            } else { glutin::ControlFlow::Continue }
        });
    }

    fn present<G, C>(&mut self, canvas: &C, glyph_lib: &GlyphLibrary<G, TextureRegion>)
    where
        G: tcrab_console::canvas::CustomGlyph,
        C: tcrab_console::Canvas<G>,
    {
        let (width_cells, height_cells) = canvas.size();
        let window_width = width_cells as u32 * self.cell_width;
        let window_height = height_cells as u32 * self.cell_height;
        self.windowed_context.window().set_inner_size(
            glutin::dpi::LogicalSize::new(window_width as f64, window_height as f64));
        self.windowed_context.window().show();
        unsafe {
            glcheck!(gl::ClearColor(1.0, 0.0, 1.0, 1.0));
            glcheck!(gl::Clear(gl::COLOR_BUFFER_BIT));

            glcheck!(gl::Viewport(0, 0, window_width as GLsizei, window_height as GLsizei));
            glcheck!(gl::UseProgram(self.shader_program));
            let projection_matrix = gfx::ortho_matrix(
                0.0, window_width as f32,
                0.0, window_height as f32,
                -0.1, 1.0,
            );
            glcheck!(gl::UniformMatrix4fv(
                self.projection_uniform_location,
                1,
                gl::FALSE,
                projection_matrix.as_ptr(),
            ));

            glcheck!(gl::BindVertexArray(self.vertex_array_object));

            let geom = gfx::gen_canvas_geometry(
                canvas,
                self.cell_width,
                self.cell_height,
                self.texture_width,
                self.texture_height,
                glyph_lib,
            );
            glcheck!(gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object));
            glcheck!(gl::BufferData(
                gl::ARRAY_BUFFER,
                (geom.vertices.len() * std::mem::size_of::<gfx::Vertex>()) as GLsizeiptr,
                geom.vertices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            ));
            glcheck!(gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer_object));
            glcheck!(gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (geom.indices.len() * std::mem::size_of::<gfx::Index>()) as GLsizeiptr,
                geom.indices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            ));

            glcheck!(gl::ActiveTexture(gl::TEXTURE0));
            glcheck!(gl::Uniform1i(self.texture_uniform_location, 0));
            glcheck!(gl::BindTexture(gl::TEXTURE_2D, self.texture));

            glcheck!(gl::DrawElements(
                gl::TRIANGLES,
                geom.indices.len() as GLsizei,
                gfx::GL_INDEX_TYPE,
                std::ptr::null(),
            ));
        }
        self.windowed_context.swap_buffers().unwrap();
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        // We'll be nice and clean up our OpenGL stuff :)
        unsafe {
            glcheck!(gl::DeleteTextures(1, &self.texture));
            glcheck!(gl::DeleteVertexArrays(1, &self.vertex_array_object));
            glcheck!(gl::DeleteBuffers(1, &self.vertex_buffer_object));
            glcheck!(gl::DeleteBuffers(1, &self.index_buffer_object));
            glcheck!(gl::DeleteProgram(self.shader_program));
        }
    }
}
