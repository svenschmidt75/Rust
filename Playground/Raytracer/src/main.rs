extern crate primitives;
extern crate sdl2;

use std::f64;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;

use primitives::Color::Color;
use primitives::Ray::Ray;
use primitives::Shape::Shape;
use primitives::Sphere::Sphere;
use primitives::Vector4f::Vector4f;
use primitives::Vertex4f::Vertex4f;

// How to setup SDL2: https://github.com/AngryLawyer/rust-sdl2#sdl20--development-libraries
// Note: Use the VC ones, NOT the mingw ones!


fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let width = 400;
    let height = 200;

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
    let mut pixel_data: Vec<u8> = vec![0; (width * height * 4) as usize];


    /* Raytrace a scene
     *
     * The camera coordinate system is right-handed, with x pointing to the right, y pointing up and
     * the negative z axis pointing into the screen.
     * The camera is positioned at (0, 0, 0), the display screen at z=-1 with x in [-1, -1]
     * and y in [-1, -1].
     */
    let upper_left_corner = Vertex4f::new(-2.0, 1.0, -1.0, 0.0);
    let vertical = Vector4f::new(0.0, -2.0, 0.0, 0.0);
    let horizontal = Vector4f::new(4.0, 0.0, 0.0, 0.0);
    let camera_origin = Vertex4f::new(0.0, 0.0, 0.0, 0.0);

    // scene objects
    let sphere = Sphere::new(Color::new(1.0, 0.0, 0.0), 0.5, Vertex4f::new(0.0, 0.0, -1.0, 0.0));

    for x in 0..width {
        for y in 0..height {
            let u = x as f64 / width as f64;
            let v = y as f64 / height as f64;
            let ray_target = upper_left_corner + u * horizontal + v * vertical;
            let ray = Ray::new(camera_origin, ray_target.as_vector());
            let intersection_points = sphere.intersect(&ray, 0.0, f64::MAX);
            let color = if intersection_points.len() > 0 {
                let t = intersection_points[1];
                let intersection = ray.point_on_ray(t);
                let sphere_normal = sphere.getNormalAt(&intersection);
                Color::new(0.5 * (sphere_normal.x + 1.), 0.5 * (sphere_normal.y + 1.), 0.5 * (sphere_normal.z + 1.))
            } else {
                color(&ray)
            };

            let index = ((y * width + x) * 4) as usize;
            pixel_data[index] = 1;   // A
            pixel_data[index + 1] = (255.99 * color.b) as u8; // B
            pixel_data[index + 2] = (255.99 * color.g) as u8; // G
            pixel_data[index + 3] = (255.99 * color.r) as u8; // R
        }
    }

    texture.update(None, &pixel_data, (width * 4) as usize).unwrap();
    renderer.copy(&texture, None, None).unwrap();
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

fn color(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.y + 1.0);
    let color_vector = (1.0 - t) * Vector4f::new(1.0, 1.0, 1.0, 0.0) + t * Vector4f::new(0.5, 0.7, 1.0, 0.0);
    Color::new(color_vector.x, color_vector.y, color_vector.z)
}
