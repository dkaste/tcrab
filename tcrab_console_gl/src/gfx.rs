use gl::types::{GLenum, GLuint, GLchar, GLint, GLsizei};

use crate::TextureRegion;

pub type Index = u16;
pub const GL_INDEX_TYPE: GLenum = gl::UNSIGNED_SHORT;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coord: [f32; 2],
    pub foreground_color: [f32; 3],
    pub background_color: [f32; 3],
}

impl Vertex {
    pub unsafe fn setup_vertex_array_object(vao: GLuint, vbo: GLuint) {
        let vertex_size = std::mem::size_of::<Self>() as GLsizei;
        glcheck!(gl::BindVertexArray(vao));
        glcheck!(gl::BindBuffer(gl::ARRAY_BUFFER, vbo));
        // Position
        glcheck!(gl::EnableVertexAttribArray(0));
        glcheck!(gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            vertex_size,
            0 as *const _,
        ));
        // Tex coord
        glcheck!(gl::EnableVertexAttribArray(1));
        glcheck!(gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            vertex_size,
            std::mem::size_of::<[f32; 2]>() as *const _,
        ));
        // Foreground color
        glcheck!(gl::EnableVertexAttribArray(2));
        glcheck!(gl::VertexAttribPointer(
            2,
            3,
            gl::FLOAT,
            gl::FALSE,
            vertex_size,
            std::mem::size_of::<[f32; 4]>() as *const _,
        ));
        // Background color
        glcheck!(gl::EnableVertexAttribArray(3));
        glcheck!(gl::VertexAttribPointer(
            3,
            3,
            gl::FLOAT,
            gl::FALSE,
            vertex_size,
            std::mem::size_of::<[f32; 7]>() as *const _,
        ));
    }
}

pub struct CanvasGeometry {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

pub fn gen_canvas_geometry<G, C>(
    canvas: &C,
    cell_width: u32,
    cell_height: u32,
    texture_width: u32,
    texture_height: u32,
    glyph_lib: &tcrab_console::canvas::GlyphLibrary<G, TextureRegion>,
) -> CanvasGeometry
where
    G: tcrab_console::canvas::CustomGlyph,
    C: tcrab_console::Canvas<G>,
{
    let cell_width = cell_width as f32;
    let cell_height = cell_height as f32;
    let texture_width = texture_width as f32;
    let texture_height = texture_height as f32;
    let (width_cells, height_cells) = canvas.size();
    let mut vertices = Vec::with_capacity(width_cells * height_cells * 4);
    let mut indices = Vec::with_capacity(width_cells * height_cells * 6);
    for cell_y in 0..height_cells {
        let y = (height_cells - cell_y - 1) as f32 * cell_height;
        for cell_x in 0..width_cells {
            let x = cell_x as f32 * cell_width;
            let cell = canvas.get_cell(cell_x, cell_y);
            let tex_region = glyph_lib.get_glyph_def(cell.glyph);
            let foreground_color = cell.foreground_color.to_rgb_f32();
            let background_color = cell.background_color.to_rgb_f32();
            let i = vertices.len() as u16;
            let tex_x = tex_region.x as f32 / texture_width;
            let tex_y = tex_region.y as f32 / texture_height;
            let tex_w = tex_region.width as f32 / texture_width;
            let tex_h = tex_region.height as f32 / texture_height;
            indices.extend_from_slice(&[i, i + 1, i + 2, i, i + 3, i + 1]);
            vertices.extend_from_slice(&[
                Vertex {
                    position: [x, y + cell_height],
                    tex_coord: [tex_x, tex_y],
                    foreground_color,
                    background_color,
                },
                Vertex {
                    position: [x + cell_width, y],
                    tex_coord: [tex_x + tex_w, tex_y + tex_h],
                    foreground_color,
                    background_color,
                },
                Vertex {
                    position: [x, y],
                    tex_coord: [tex_x, tex_y + tex_h],
                    foreground_color,
                    background_color,
                },
                Vertex {
                    position: [x + cell_width, y + cell_height],
                    tex_coord: [tex_x + tex_w, tex_y],
                    foreground_color,
                    background_color,
                },
            ]);
        }
    }
    CanvasGeometry { vertices, indices }
}

pub fn ortho_matrix(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> [f32; 16] {
    [
        2.0 / (right - left), 0.0, 0.0, 0.0,
        0.0, 2.0 / (top - bottom), 0.0, 0.0,
        0.0, 0.0, -2.0 / (far - near), 0.0,

        -(right + left) / (right - left),
        -(top + bottom) / (top - bottom),
        -(far + near) / (far - near),
        1.0,
    ]
}

pub unsafe fn link_program(vert_src: &[u8], frag_src: &[u8]) -> GLuint {
    let shader_program = glcheck!(gl::CreateProgram());
    let vertex_shader = compile_shader(gl::VERTEX_SHADER, vert_src);
    let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, frag_src);
    glcheck!(gl::AttachShader(shader_program, vertex_shader));
    glcheck!(gl::AttachShader(shader_program, fragment_shader));
    // Linking should theoretically fail only if the embedded shaders are incorrect.
    glcheck!(gl::LinkProgram(shader_program));
    glcheck!(gl::DetachShader(shader_program, vertex_shader));
    glcheck!(gl::DetachShader(shader_program, fragment_shader));
    glcheck!(gl::DeleteShader(vertex_shader));
    glcheck!(gl::DeleteShader(fragment_shader));
    shader_program
}

// Since the only shaders we will compile are embedded in the library, we will ignore compile error
// checking. Make sure the included shaders are correct :)
unsafe fn compile_shader(type_: GLenum, src: &[u8]) -> GLuint {
    let shader = glcheck!(gl::CreateShader(type_));
    let src_ptr = src.as_ptr() as *const GLchar;
    let src_len = src.len() as GLint;
    glcheck!(gl::ShaderSource(shader, 1, &src_ptr, &src_len));
    glcheck!(gl::CompileShader(shader));
    shader
}