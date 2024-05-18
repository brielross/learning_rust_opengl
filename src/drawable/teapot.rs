use std::fs;

use glium::{glutin::surface::WindowSurface, DrawParameters, Surface, VertexBuffer};

mod teapot_data;

pub struct Teapot {
    positions: VertexBuffer<teapot_data::Vertex>,
    normals: VertexBuffer<teapot_data::Normal>,
    indices: glium::IndexBuffer<u16>,
    program: glium::Program,
}

impl Teapot {
    pub fn new(display: glium::backend::glutin::Display<WindowSurface>) -> Teapot {
        let positions = glium::VertexBuffer::new(&display, &teapot_data::VERTICES).unwrap();
        let normals = glium::VertexBuffer::new(&display, &teapot_data::NORMALS).unwrap();
        let indices = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &teapot_data::INDICES,
        )
        .unwrap();

        let use_gourad = false;

        let v_gourad_src = "src/shaders/gourad_vertex.glsl";
        let v_blinn_phong_src = "src/shaders/blinn_phong_vertex.glsl";
        let vertex_shader_file = if use_gourad {
            v_gourad_src
        } else {
            v_blinn_phong_src
        };
        let vertex_shader_src = fs::read_to_string(vertex_shader_file).unwrap();
        let vertex_shader_src = vertex_shader_src.as_str();

        let f_gourad_src = "src/shaders/gourad_fragment.glsl";
        let f_blinn_phong_src = "src/shaders/blinn_phong_fragment.glsl";
        let fragment_shader_file = if use_gourad {
            f_gourad_src
        } else {
            f_blinn_phong_src
        };
        let fragment_shader_src = fs::read_to_string(fragment_shader_file).unwrap();
        let fragment_shader_src = fragment_shader_src.as_str();

        let program =
            glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        return Teapot {
            positions,
            normals,
            indices,
            program,
        };
    }

    pub fn draw_teapot(
        &self,
        target: &mut glium::Frame,
        params: DrawParameters,
        perspective: [[f32; 4]; 4],
        view: [[f32; 4]; 4],
    ) {
        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        let light = [-5.5, 0.4, 0.9f32];

        target
            .draw(
                (&self.positions, &self.normals),
                &self.indices,
                &self.program,
                &uniform! { model: model, view: view, u_light: light, perspective: perspective },
                &params,
            )
            .unwrap();
    }
}
