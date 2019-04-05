#[allow(dead_code)]

extern crate sdl2;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{TextureAccess};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;

mod lib;

use lib::world::World;

fn copy_world_to_screen(pixel_data: &mut [u8], world: &mut World, width: u32, height: u32) {
    for row in 0..height {
        for col in 0..width {
            let index = ((row * width + col) * 4) as usize;
            let color = match world.is_alive(row, col) {
                true => Color::RGB(255, 255, 255),
                false => Color::RGB(0, 0, 0),
            };
            pixel_data[index] = 1;              // A
            pixel_data[index + 1] = color.b;    // B
            pixel_data[index + 2] = color.g;    // G
            pixel_data[index + 3] = color.r;    // R
        }
    }
}

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let width = 800;
    let height = 600;

    // Create the window
    let window = video.window("Conway's Game Of Life", width, height)
        .position_centered()
        .build().unwrap();

    let mut renderer = window.into_canvas()
        .build().unwrap();

    renderer.clear();

    // create texture we render in
    let texture_creator = renderer.texture_creator();
    let mut texture = texture_creator.create_texture(PixelFormatEnum::RGBA8888, TextureAccess::Streaming, width, height).unwrap();

    // pixel data for texture
    let mut pixel_data: Vec<u8> = vec![0; (width * height * 4) as usize];

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut world = World::new(width, height);
//    world.blinker_period_2(10, 10);
//    world.pulsar_period_3(100, 100);
//    world.glider(140, 100);
//    world.pentadecathlon(140, 120);
    world.init_random();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {
                    world = world.evolve();
                    copy_world_to_screen(&mut pixel_data, &mut world, width, height);
                    texture.update(None, &pixel_data, (width * 4) as usize).unwrap();
                    renderer.copy(&texture, None, None).unwrap();
                    renderer.present();
//                    thread::sleep(::std::time::Duration::new(0, 100_00));
                }
            }
        }
    }
}
