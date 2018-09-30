extern crate gl;

use components::renderable::{Renderable, VertexRange};
use context::Context;
use gl::types::{GLboolean, GLfloat, GLsizei, GLsizeiptr, GLuint, GLvoid};
use std::cmp;
use std::collections::BTreeSet;
use std::ffi::CString;
use std::mem;
use std::ptr;
use util;

static VS_SRC: &'static str = "
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec4 aRect;

uniform mat4 uViewProjection;

out vec4 vertexColor;

void main() {
    vec4 pos = vec4(aPos, 1.0);
    
    mat4 scale = mat4(1.0);
    scale[0][0] = aRect[2];
    scale[1][1] = aRect[3];

    mat4 translate = mat4(1.0);
    translate[3][0] = aRect[0];
    translate[3][1] = aRect[1];

    gl_Position = uViewProjection * translate * scale * pos;
    vertexColor = vec4(aColor, 1.0);
}";

static FS_SRC: &'static str = "
#version 330 core
out vec4 FragColor;

in vec4 vertexColor;

void main() {
    FragColor = vertexColor;
}";

const VERTEX_SIZE: i32 = 10;
const VERTS_PER_OBJECT: usize = 4;
const INDICES_PER_OBJECT: usize = 6;
const NUM_OBJECTS: usize = 5120;

pub struct System {
    vertex_ranges: BTreeSet<Box<VertexRange>>,
    index_ranges: BTreeSet<Box<VertexRange>>,

    program: u32,

    // Vertex arrays
    vao: u32,

    // Vertex buffers
    vbo: u32,
    vbo_size: usize,

    // Element buffers
    ebo: u32,
    ebo_size: usize,

    // Uniforms
    u_view_projection: i32,
}

impl System {
    pub fn new() -> System {
        return System {
            vertex_ranges: BTreeSet::new(),
            index_ranges: BTreeSet::new(),

            program: 0,

            vao: 0,

            vbo: 0,
            vbo_size: NUM_OBJECTS * VERTS_PER_OBJECT * VERTEX_SIZE as usize,

            ebo: 0,
            ebo_size: NUM_OBJECTS * VERTS_PER_OBJECT * 3 as usize,

            u_view_projection: 0,
        };
    }
}

impl ::systems::System for System {
    fn initialize(&mut self) {
        self.program = unsafe {
            let vs = util::shader::compile_shader(VS_SRC, gl::VERTEX_SHADER);
            let fs = util::shader::compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
            let program = util::shader::link_program(vs, fs);
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            self.u_view_projection = gl::GetUniformLocation(
                program,
                CString::new("uViewProjection").unwrap().as_ptr() as *const i8,
            );

            program
        };

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vbo_size * mem::size_of::<GLfloat>()) as GLsizeiptr,
                ptr::null(),
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut self.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.ebo_size * mem::size_of::<GLuint>()) as GLsizeiptr,
                ptr::null(),
                gl::STATIC_DRAW,
            );
        }
    }

    fn apply(&mut self, context: &mut Context) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::UseProgram(self.program);

            let view_projection = &context.camera.view_matrix;
            gl::UniformMatrix4fv(
                self.u_view_projection,
                1,
                gl::TRUE as GLboolean,
                mem::transmute(&view_projection[0]),
            );
        }

        let mut max_index = 0;
        for current_index in 0..context.entities.raw_len() {
            let entity = match context.entities.get_raw(current_index) {
                Some(entity) => entity,
                None => continue,
            };

            let renderable = match entity.get_component::<Renderable>(Renderable::name()) {
                Some(renderable) => renderable,
                None => continue,
            };

            match &renderable.index_range {
                Some(range) => max_index = cmp::max(max_index, range.start + range.length),
                None => {}
            }

            self.render(renderable);
        }

        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                VERTEX_SIZE * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                VERTEX_SIZE * mem::size_of::<GLfloat>() as GLsizei,
                (3 * mem::size_of::<GLfloat>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                2,
                4,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                VERTEX_SIZE * mem::size_of::<GLfloat>() as GLsizei,
                (6 * mem::size_of::<GLfloat>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(2);

            gl::DrawElements(
                gl::TRIANGLES,
                max_index as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}

impl System {
    fn render(&mut self, renderable: &mut Renderable) {
        if !renderable.dirty {
            return;
        }

        if renderable.vertex_range.is_none() {
            self.assign_vertex_range(renderable);
        }

        unsafe {
            match renderable.vertex_range {
                Some(ref range) => {
                    let quad: [[GLfloat; 6]; 4] = [
                        [-0.5, 0.5, 0.0, 1.0, 0.0, 0.0],  //
                        [0.5, 0.5, 0.0, 0.0, 1.0, 0.0],   //
                        [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0], //
                        [0.5, -0.5, 0.0, 1.0, 0.0, 0.0],  //
                    ];

                    for (i, vertex) in quad.iter().enumerate() {
                        let offset = (range.start + i) * VERTEX_SIZE as usize;
                        gl::BufferSubData(
                            gl::ARRAY_BUFFER,
                            (offset * mem::size_of::<GLfloat>()) as GLsizeiptr,
                            (vertex.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                            mem::transmute(&vertex[0]),
                        );

                        let rect: [GLfloat; 4] = [
                            renderable.x,
                            renderable.y,
                            renderable.width,
                            renderable.height,
                        ];

                        gl::BufferSubData(
                            gl::ARRAY_BUFFER,
                            ((offset + vertex.len()) * mem::size_of::<GLfloat>()) as GLsizeiptr,
                            (rect.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                            mem::transmute(&rect[0]),
                        );
                    }
                }
                None => (),
            };

            match renderable.index_range {
                Some(ref range) => {
                    // Put a quad up on that GPU
                    let triange_start = (range.start as u32 / 6) * 4;
                    let indices: [GLuint; 6] = [
                        triange_start,
                        triange_start + 1,
                        triange_start + 2,
                        triange_start + 1,
                        triange_start + 2,
                        triange_start + 3,
                    ];
                    gl::BufferSubData(
                        gl::ELEMENT_ARRAY_BUFFER,
                        (range.start * mem::size_of::<GLuint>()) as GLsizeiptr,
                        (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                        mem::transmute(&indices[0]),
                    );
                }
                None => (),
            };
        }

        renderable.dirty = false;
    }

    fn assign_vertex_range(&mut self, renderable: &mut Renderable) {
        let num_vertices = VERTS_PER_OBJECT;
        let vertex_range = self.find_free_range(&self.vertex_ranges, num_vertices);
        renderable.vertex_range = Some(Box::new(vertex_range.clone()));
        self.vertex_ranges.insert(Box::new(vertex_range));

        let num_indices = INDICES_PER_OBJECT;
        let index_range = self.find_free_range(&self.index_ranges, num_indices);
        renderable.index_range = Some(Box::new(index_range.clone()));
        self.index_ranges.insert(Box::new(index_range));
    }

    fn find_free_range(&self, ranges: &BTreeSet<Box<VertexRange>>, size: usize) -> VertexRange {
        let mut first_free_index = 0;
        for range in ranges.iter() {
            if range.start - first_free_index > size {
                break;
            }
            first_free_index = range.start + range.length;
        }

        return VertexRange {
            start: first_free_index,
            length: size,
        };
    }
}
