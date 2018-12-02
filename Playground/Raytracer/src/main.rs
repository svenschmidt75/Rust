extern crate sdl2;

use std::thread;
use std::time::Duration;

use sdl2::render::{Texture, TextureAccess};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use Vertex4f::{Vertex4f};


// How to setup SDL2: https://github.com/AngryLawyer/rust-sdl2#sdl20--development-libraries
// Note: Use the VC ones, NOT the mingw ones!


fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let width = 600;
    let height = 480;

    // Create the window
    let window = video.window("Sven's Raytracer", width, height)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.into_canvas()
        .accelerated()
        .build().unwrap();

    // Render a fully black window
//    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    // create texture we render in
    let texture_creator = renderer.texture_creator();
    let mut texture = texture_creator.create_texture(PixelFormatEnum::RGBA8888, TextureAccess::Static, width, height).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // pixel data for texture
    let mut pixel_data = vec![0; (width * height * 4) as usize];



    /* Raytrace a scene
     *
     * The camera coordinate system is right-handed, with x pointing to the right, y pointing up and
     * the negative z axis pointing into the screen.
     * The camera is positioned at (0, 0, 0), the display screen at z=-1 with x in [-1, -1]
     * and y in [-1, -1].
     */
    let upper_left_corner = Vertex4f::new();




    // create scene objects

    // render
    for i in 0..(width * height) as usize {
        let index = i * 4;
        pixel_data[index] = 1;   // A
        pixel_data[index + 1] = 77;  // B
        pixel_data[index + 2] = 171; // G
        pixel_data[index + 3] = 243; // R
    }

    texture.update(None, &pixel_data, (width * 4) as usize);
    renderer.copy(&texture, None, None);
    renderer.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {},
            }
        }
    }
}
