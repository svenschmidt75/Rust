use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture};
use sfml::window::{ContextSettings, Event, Style, VideoMode};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let settings = ContextSettings::default();
    let mut window = RenderWindow::new(
        VideoMode::new(WIDTH, HEIGHT, 32),
        "Software 3D Renderer",
        Style::DEFAULT,
        &settings,
    ).expect("Failed to create SFML window");

    window.set_vertical_sync_enabled(true);

    let mut texture = Texture::new().expect("Failed to create texture object");
    texture.create(WIDTH, HEIGHT).expect("Failed to allocate VRAM for texture");

    let mut framebuffer = vec![0u8; (WIDTH * HEIGHT * 4) as usize];
    let mut timer: u8 = 0;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        // --- SOFTWARE RENDERING PHASE ---
        timer = timer.wrapping_add(1);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = ((y * WIDTH + x) * 4) as usize;
                framebuffer[index]     = x.wrapping_add(timer as u32) as u8;
                framebuffer[index + 1] = y.wrapping_add(timer as u32) as u8;
                framebuffer[index + 2] = 150;
                framebuffer[index + 3] = 255;
            }
        }

        // --- DISPLAY PHASE ---

        // 1. Update the texture.
        // No sprite exists yet that "borrows" the texture, so this is allowed.
        unsafe {
            texture.update_from_pixels(&framebuffer, WIDTH, HEIGHT, 0, 0);
        }

        // 2. Create the sprite locally for this frame.
        // The "borrow" of texture starts here...
        let mut sprite = Sprite::with_texture(&texture);

        window.clear(Color::BLACK);
        window.draw(&sprite);
        window.display();

        // ...and the "borrow" ends here when sprite goes out of scope at the end of the loop!
    }
}