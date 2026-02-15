mod camera;
mod cube;
mod lin_alg;
mod matrix4;
mod render_context;
mod renderable;
mod triangle;
mod vertex;

use std::f32::consts::PI;
use std::time::{Duration, Instant};
use crate::camera::Camera;
use crate::cube::UnitCube;
use crate::renderable::Renderable;
use crate::vertex::Vertex4;
use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Sprite, Text, Texture, Transformable};
use sfml::system::{Vector2f, Vector2u};
use sfml::window::window_enums::State;
use sfml::window::{ContextSettings, Event, Style, VideoMode};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let settings = ContextSettings::default();

    let mut window = RenderWindow::new(
        VideoMode::new(Vector2u::new(WIDTH, HEIGHT), 32),
        "Software 3D Renderer (SFML 3)",
        Style::DEFAULT,
        State::Windowed,
        &settings,
    )
    .expect("Failed to create SFML window");
    window.set_vertical_sync_enabled(true);

    // SS: for displaying FPS
    let font = Font::from_file("/System/Library/Fonts/Helvetica.ttc")
        .expect("Could not find system font");
    let mut fps_text = Text::new("FPS: 0", &font, 20);
    fps_text.set_fill_color(Color::GREEN);

    // 1. Create the texture object
    let mut texture = Texture::new().expect("Failed to create texture object");

    // 2. Resize it.
    // Argument 1: Size
    // Argument 2: sRGB (false is standard for software rendering)
    if !texture.resize(Vector2u::new(WIDTH, HEIGHT), false) {
        panic!("Failed to allocate texture memory");
    }

    let mut ctx = render_context::RenderContext::new(WIDTH, HEIGHT);
    ctx.set_camera(Camera::new(
        Vertex4::new_vertex(0f32, 0f32, 1f32),
        Vertex4::new_vector(0f32, 0f32, -1f32),
        Vertex4::new_vector(0f32, 1f32, 0f32),
    ));

    ctx.set_camera(Camera::from_look_at(3.0, PI / 4.0, PI / 4.0));

    ctx.orthographic(-2.0, 2.0, -2.0, 2.0, -2.0, 2.0);
    ctx.perspective(-2.0, 2.0, -2.0, 2.0, -2.0, 2.0);

    let mut timer: u8 = 0;
    let mut frame_count = 0;
    let mut total_time = 0f32;

    // SS: we use delta to control the speed at which we want to advance
    // for example a rotation angle in an FPS-independent way.
    let mut delta = Duration::new(0, 0);

    // SS: instantiate timing object
    let mut last_time = Instant::now();

    let cube = UnitCube::new();

    // SS: add rotation around world z-axis
//    cube.add_transform(RotationZ::new, 90.0);

    // --- MAIN LOOP ---
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if let Event::Closed = event {
                window.close();
            }
        }

        // --- SOFTWARE RENDERING PHASE ---
        timer = timer.wrapping_add(1);

        // SS: capture time it takes to render the frame
        let current_time = Instant::now();

        // SS: render scene
        cube.render(&mut ctx, delta.as_secs_f32());

        // SS: time it took to render frame
        delta = current_time.duration_since(last_time);
        last_time = current_time;

        // SS: duration in milliseconds
        let delta_ms = delta.as_secs_f32() * 1000.0;

        frame_count += 1;
        total_time += delta_ms;

        if total_time > 1000.0 {
            // SS: more than one second has passed, update FPS
            let fps = frame_count as f32 / (total_time / 1000.0);
            fps_text.set_string(&format!("FPS: {:.1}", fps));
//            println!("FPS: {}", fps);

            // SS: display in top-right corner of the render window
            let window_size = window.size();
            let text_bounds = fps_text.global_bounds();
            let x_pos = window_size.x as f32 - text_bounds.size.x - 20.0;
            let y_pos = 10.0; // Top margin
            fps_text.set_position(Vector2f::new(x_pos, y_pos));

            total_time = 0.0;
            frame_count = 0;
        }

        // --- DISPLAY PHASE ---
        // Update the pixels
        texture.update_from_pixels(
            &ctx.framebuffer,
            Vector2u::new(WIDTH, HEIGHT),
            Vector2u::new(0, 0),
        );

        // Create sprite inside the loop to release the borrow every frame
        let sprite = Sprite::with_texture(&texture);

        window.clear(Color::BLACK);
        window.draw(&sprite);
        window.draw(&fps_text);
        window.display();
    }
}
