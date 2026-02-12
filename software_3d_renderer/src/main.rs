mod camera;
mod lin_alg;
mod matrix4;
mod render_context;
mod renderable;
mod triangle;
mod vertex;

use crate::camera::Camera;
use crate::renderable::Renderable;
use crate::triangle::Triangle;
use crate::vertex::Vertex4;
use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture};
use sfml::system::Vector2u;
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
        Vertex4::new_vertex(1f32, 0f32, 0f32),
        Vertex4::new_vector(0f32, 0f32, -1f32),
        Vertex4::new_vector(0f32, 1f32, 0f32),
    ));

    let mut timer: u8 = 0;

    // --- MAIN LOOP ---
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if let Event::Closed = event {
                window.close();
            }
        }

        // --- SOFTWARE RENDERING PHASE ---
        timer = timer.wrapping_add(1);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                //                ctx.set_pixel(x, y, x.wrapping_add(timer as u32) as u8, y.wrapping_add(timer as u32) as u8, 150, 255);
            }
        }

        let t1 = Triangle::new([
            Vertex4::new(-1f32, 1f32, 0f32, 0f32),
            Vertex4::new(0f32, 0f32, 0f32, 0f32),
            Vertex4::new(-1f32, -1f32, 0f32, 0f32),
        ]);
        t1.render(&mut ctx);

        let t2 = Triangle::new([
            Vertex4::new(0f32, 0f32, 0f32, 0f32),
            Vertex4::new(1f32, 1f32, 0f32, 0f32),
            Vertex4::new(1f32, -1f32, 0f32, 0f32),
        ]);
        t2.render(&mut ctx);

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
        window.display();
    }
}
