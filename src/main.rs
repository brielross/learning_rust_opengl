use glium::Surface;
use std::time::Instant;

mod drawables;

#[macro_use]
extern crate glium;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let mut t: f32 = 0.0;

    let start_time = Instant::now();
    let mut frames = 0;

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
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    drawables::draw_image(display.clone(), &mut target);
                    drawables::draw_triangle(display.clone(), &mut target, t);
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
