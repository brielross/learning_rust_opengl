use glium::Surface;
use std::time::Instant;

#[path = "drawable/teapot.rs"]
mod teapot;

#[macro_use]
extern crate glium;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let start_time = Instant::now();
    let mut frames = 0;

    let teapot = teapot::Teapot::new(display.clone());

    let mut t = 0f32;

    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                    let avg_fps = frames / start_time.elapsed().as_secs();
                    println!("Average FPS: {}", avg_fps);
                }
                winit::event::WindowEvent::RedrawRequested => {
                    t += 0.02;
                    let mut target = display.draw();
                    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            ..Default::default()
                        },
                        // Enables backface culling
                        //backface_culling:
                        //    glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                        ..Default::default()
                    };
                    let perspective = get_perspective_matrix(&window);
                    let view = get_view_matrix(
                        &[3.0 * t.cos(), 1.5 * t.cos(), 3.0 * t.sin()],
                        &[-1.0 * t.cos(), -0.5 * t.cos(), -1.0 * t.sin()],
                        &[0.0, 1.0, 0.0],
                    );
                    teapot.draw_teapot(&mut target, params, perspective, view);
                    target.finish().unwrap();
                    frames += 1;
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        };
    });
}

fn get_perspective_matrix(window: &winit::window::Window) -> [[f32; 4]; 4] {
    let perspective = {
        let window_size = window.inner_size();
        let width = window_size.width;
        let height = window_size.height;
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 3.141592 / 3.0;
        // Max and min depth values
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    };
    return perspective;
}

fn get_view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
