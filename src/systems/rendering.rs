extern crate gl;

use components::renderable::{Renderable, VertexRange};
use entities::Entity;
use gl::types::{GLboolean, GLfloat, GLint, GLsizei, GLsizeiptr, GLvoid};
use std::cmp;
use std::collections::BTreeSet;
use std::mem;
use std::ptr;
use util;

static VS_SRC: &'static str = "
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec4 aTransformX;
layout (location = 3) in vec4 aTransformY;
layout (location = 4) in vec4 aTransformZ;
layout (location = 5) in vec4 aTransformW;

out vec4 vertexColor;

void main() {
    vec4 pos = vec4(aPos, 1.0);
    gl_Position = vec4(
        dot(pos, aTransformX), 
        dot(pos, aTransformY), 
        dot(pos, aTransformZ), 
        dot(pos, aTransformW)
    );
    vertexColor = vec4(aColor, 1.0);
}";

static FS_SRC: &'static str = "
#version 330 core
out vec4 FragColor;

in vec4 vertexColor;

void main() {
    FragColor = vertexColor;
}";

const VERTEX_SIZE: i32 = 22;
const VERTS_PER_OBJECT: usize = 4;
const INDICES_PER_OBJECT: usize = 6;

pub struct System {
    vertex_ranges: BTreeSet<Box<VertexRange>>,
    index_ranges: BTreeSet<Box<VertexRange>>,

    program: u32,

    vao: u32,

    vbo: u32,
    vbo_size: usize,

    ebo: u32,
    ebo_size: usize,
}

impl System {
    pub fn new() -> System {
        return System {
            vertex_ranges: BTreeSet::new(),
            index_ranges: BTreeSet::new(),

            program: 0,

            vao: 0,

            vbo: 0,
            vbo_size: 1024 * VERTS_PER_OBJECT * VERTEX_SIZE as usize,

            ebo: 0,
            ebo_size: 1024 * VERTS_PER_OBJECT * 3 as usize,
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
                (self.ebo_size * mem::size_of::<GLint>()) as GLsizeiptr,
                ptr::null(),
                gl::STATIC_DRAW,
            );
        }
    }

    fn apply(&mut self, entities: &mut [&mut Entity]) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::UseProgram(self.program);
        }

        let mut max_index = 0;
        for entity in entities {
            let renderable = match entity.get_renderable() {
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

            for i in 0..4 {
                gl::VertexAttribPointer(
                    2 + i,
                    4,
                    gl::FLOAT,
                    gl::FALSE as GLboolean,
                    VERTEX_SIZE * mem::size_of::<GLfloat>() as GLsizei,
                    ((6 + 4 * i as usize) * mem::size_of::<GLfloat>()) as *const GLvoid,
                );
                gl::EnableVertexAttribArray(2 + i);
            }

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
        println!("Rendering {:?}", renderable);
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
                        let offset = range.start + (i * 22);
                        gl::BufferSubData(
                            gl::ARRAY_BUFFER,
                            (offset * mem::size_of::<GLfloat>()) as GLsizeiptr,
                            (vertex.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                            mem::transmute(&vertex[0]),
                        );

                        let local_matrix = &renderable.local_matrix;
                        gl::BufferSubData(
                            gl::ARRAY_BUFFER,
                            ((offset + 6) * mem::size_of::<GLfloat>()) as GLsizeiptr,
                            (local_matrix.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                            mem::transmute(&local_matrix[0]),
                        );
                    }
                }
                None => (),
            };

            match renderable.index_range {
                Some(ref range) => {
                    // Put a quad up on that GPU
                    let start = range.start as i32;
                    let indices: [GLint; 6] =
                        [start, start + 1, start + 2, start + 1, start + 2, start + 3];
                    gl::BufferSubData(
                        gl::ELEMENT_ARRAY_BUFFER,
                        (range.start * mem::size_of::<GLint>()) as GLsizeiptr,
                        (indices.len() * mem::size_of::<GLint>()) as GLsizeiptr,
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
