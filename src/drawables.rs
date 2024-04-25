use glium::glutin::surface::WindowSurface;
use glium::Surface;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct TextureVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub fn draw_image(
    display: glium::backend::glutin::Display<WindowSurface>,
    target: &mut glium::Frame,
) {
    let image = image::load(
        std::io::Cursor::new(&include_bytes!("../assets/opengl.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    implement_vertex!(TextureVertex, position, tex_coords);
    let shape = vec![
        TextureVertex {
            position: [-0.5, -0.5],
            tex_coords: [0.0, 0.0],
        },
        TextureVertex {
            position: [0.5, -0.5],
            tex_coords: [1.0, 0.0],
        },
        TextureVertex {
            position: [0.5, 0.5],
            tex_coords: [1.0, 1.0],
        },
        TextureVertex {
            position: [0.5, 0.5],
            tex_coords: [1.0, 1.0],
        },
        TextureVertex {
            position: [-0.5, 0.5],
            tex_coords: [0.0, 1.0],
        },
        TextureVertex {
            position: [-0.5, -0.5],
            tex_coords: [0.0, 0.0],
        },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ],
        tex: &texture,
    };

    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
}

pub fn draw_triangle(
    display: glium::backend::glutin::Display<WindowSurface>,
    target: &mut glium::Frame,
    t: f32,
) {
    implement_vertex!(Vertex, position, color);
    let shape = vec![
        Vertex {
            position: [-0.5, -0.5],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.0, 0.5],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.25],
            color: [0.0, 0.0, 1.0],
        },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        out vec3 vertex_color;

        uniform mat4 matrix;

        void main() {
            vertex_color = color;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 vertex_color;
        out vec4 color;

        void main() {
            color = vec4(vertex_color, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let x_off = t.sin() * 0.5;
    let uniforms = uniform! {
        matrix: [
            [t.cos(), t.sin(), 0.0, 0.0],
            [-t.sin(), t.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x_off, 0.0, 0.0, 1.0f32],
        ]
    };

    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
}
