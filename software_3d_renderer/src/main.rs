mod camera;
mod color;
mod cube;
mod image_texture;
mod lin_alg;
mod matrix4;
mod raster_vertex;
mod render_context;
mod renderable;
mod scene_object;
mod texture_manager;
mod texture_type;
mod triangle;
mod vertex4;

use crate::camera::Camera;
use crate::cube::UnitCube;
use crate::matrix4::Matrix4;
use crate::scene_object::SceneObject;
use image_texture::ImageTexture;
use sfml::graphics::{
    Color, Font, Image, RenderTarget, RenderWindow, Sprite, Text, Texture, Transformable,
};
use sfml::system::{Vector2f, Vector2i, Vector2u};
use sfml::window::mouse::Button;
use sfml::window::window_enums::State;
use sfml::window::{ContextSettings, Event, Style, VideoMode};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

fn main() {
    let mut window_width = 800;
    let mut window_height = 600;

    let settings = ContextSettings::default();
    let mut window = RenderWindow::new(
        VideoMode::new(Vector2u::new(window_width, window_height), 32),
        "Software 3D Renderer (SFML 3)",
        Style::DEFAULT,
        State::Windowed,
        &settings,
    )
    .expect("Failed to create SFML window");
    window.set_vertical_sync_enabled(true);

    // SS: for displaying FPS
    let font =
        Font::from_file("/System/Library/Fonts/Helvetica.ttc").expect("Could not find system font");
    let mut fps_text = Text::new("FPS: 0", &font, 20);
    fps_text.set_fill_color(Color::GREEN);

    // SS: create the texture object
    let mut texture = Texture::new().expect("Failed to create texture object");
    if !texture.resize(Vector2u::new(window_width, window_height), false) {
        panic!("Failed to allocate texture memory");
    }

    let mut ctx = render_context::RenderContext::new(window_width, window_height);

    // SS: position camera
    let mut theta: f32 = 0.0;
    let mut phi: f32 = 0.0;
    let radius: f32 = 3.0;
    ctx.set_camera(Camera::from_look_at(radius, theta, phi));

    //    ctx.orthographic(-2.0, 2.0, -2.0, 2.0, -2.0, 2.0);
    ctx.perspective(-2.0, 2.0, -2.0, 2.0, -2.0, 2.0);

    let mut timer: u8 = 0;
    let mut frame_count = 0;
    let mut total_time = 0f32;

    // SS: we use delta to control the speed at which we want to advance
    // for example a rotation angle in an FPS-independent way.
    let mut delta = Duration::new(0, 0);

    // SS: instantiate timing object
    let mut last_time = Instant::now();

    // SS: load texture
    let img = Image::from_file("assets/image.png").expect("Failed to load image");
    let image_texture = ImageTexture::new(img.size().x, img.size().y, img.pixel_data());
    let texture_id = ctx.texture_manager.add_texture(image_texture);

    let cube = UnitCube::new_with_image(texture_id);
    let mut scene_object = SceneObject::new(Box::new(cube));

    // SS: add rotation around world z-axis
    let mut angle: f32 = 0.0;
    scene_object.add_transform(Box::new(move |delta| {
        //        angle += delta * 0.75;
        let mut m = Matrix4::identity();
        m[0][0] = angle.cos();
        m[0][1] = -angle.sin();
        m[1][0] = angle.sin();
        m[1][1] = angle.cos();
        m
    }));

    // let mut angle2 = 0.0;
    // scene_object.add_transform(Box::new(move |delta| {
    //     angle2 += delta * 1.05;
    //     let mut m = Matrix4::identity();
    //     m[1][1] = angle2.cos();
    //     m[1][2] = -angle2.sin();
    //     m[2][1] = angle2.sin();
    //     m[2][2] = angle2.cos();
    //     m
    // }));

    // --- MAIN LOOP ---
    let mut is_mouse_pressed = false;
    let mut last_mouse_pos = Vector2i::new(0, 0);

    // SS: spherical coordinates for the camera
    while window.is_open() {
        match window.poll_event() {
            Some(Event::Closed) => {
                window.close();
            }
            Some(Event::Resized { size }) => {
                window_width = size.x;
                window_height = size.y;

                // SS: update the view to match the new window size
                let new_view = sfml::graphics::View::with_center_and_size(
                    Vector2f::new(window_width as f32 / 2.0, window_height as f32 / 2.0),
                    Vector2f::new(window_width as f32, window_height as f32),
                );
                window.set_view(&new_view);

                // SS: reinit texture
                if !texture.resize(size, false) {
                    panic!("Failed to allocate texture memory");
                }

                // SS: reinit framebuffer
                ctx.resize(window_width, window_height);
            }
            Some(Event::MouseButtonPressed {
                button: Button::Left,
                position,
            }) => {
                is_mouse_pressed = true;
                last_mouse_pos = position;
            }
            Some(Event::MouseButtonReleased {
                button: Button::Left,
                ..
            }) => {
                is_mouse_pressed = false;
            }
            Some(Event::MouseMoved { position }) => {
                if is_mouse_pressed {
                    let delta_x = position.x - last_mouse_pos.x;
                    let delta_y = position.y - last_mouse_pos.y;

                    // SS: adjust sensitivity (lower = slower rotation)
                    let sensitivity = 0.005;
                    theta -= delta_x as f32 * sensitivity;
                    phi += delta_y as f32 * sensitivity;

                    // SS: constrain Phi to (eps, PI - eps) so the camera doesn't flip upside down at the poles
                    let epsilon = 0.1;
                    phi = phi.clamp(epsilon, PI - epsilon);

                    last_mouse_pos = position;
                }
            }
            _ => {}
        }

        // --- SOFTWARE RENDERING PHASE ---
        ctx.set_camera(Camera::from_look_at(radius, theta, phi));

        timer = timer.wrapping_add(1);

        // SS: capture time it takes to render the frame
        let current_time = Instant::now();

        // SS: clear scene
        ctx.clear_framebuffer();

        // SS: render scene
        scene_object.render(&mut ctx, delta.as_secs_f32());

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
            let y_pos = 10.0;
            fps_text.set_position(Vector2f::new(x_pos, y_pos));

            total_time = 0.0;
            frame_count = 0;
        }

        // --- DISPLAY PHASE ---
        // Update the pixels
        texture.update_from_pixels(
            &ctx.framebuffer,
            Vector2u::new(window_width, window_height),
            Vector2u::new(0, 0),
        );

        window.clear(Color::BLACK);

        // Create sprite inside the loop to release the borrow every frame
        let sprite = Sprite::with_texture(&texture);
        window.draw(&sprite);

        window.draw(&fps_text);
        window.display();
    }
}
