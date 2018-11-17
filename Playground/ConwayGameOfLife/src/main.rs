extern crate sdl2;

use std::time::{Duration};
use std::thread;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, TextureAccess};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod lib;
use lib::world::World;


fn main() {
    let mut world = World::new(10, 10);
    // TODO SS: initialize world state

    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let width = 800;
    let height = 600;

    // Create the window
    let window = video.window("Conway's Game Of Life", width, height)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.into_canvas()
        .accelerated()
        .build().unwrap();

    renderer.clear();

    // create texture we render in
    let texture_creator = renderer.texture_creator();
    let mut texture = texture_creator.create_texture(PixelFormatEnum::RGBA8888, TextureAccess::Static, width, height).unwrap();

    // pixel data for texture
    let mut pixel_data = vec![0; (width * height * 4) as usize];

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {
                    world = world.evolve();


                    // render world into pixel_data


                    texture.update(None, &pixel_data, (width * 4) as usize);
                    renderer.copy(&texture, None, None);
                    renderer.present();
                }
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }


//    for i in 0..(width * height) as usize {
//        let index = i * 4;
//        pixel_data[index] = 1;   // A
//        pixel_data[index + 1] = 77;  // B
//        pixel_data[index + 2] = 171; // G
//        pixel_data[index + 3] = 243; // R
//    }
}
