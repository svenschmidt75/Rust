extern crate primitives;
extern crate rand;
extern crate sdl2;

use std::f64;

use rand::{Open01, random};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;

use primitives::Camera::Camera;
use primitives::Color::Color;
use primitives::Ray::Ray;
use primitives::Shape::Shape;
use primitives::ShapeList::ShapeList;
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


    // Raytrace a scene
    let camera = Camera::new();

    // scene objects
    let mut shapes = Vec::<Box<Shape>>::new();
    let sphere = Sphere::new(Color::new(1.0, 0.0, 0.0), 0.5, Vertex4f::new(0.0, 0.0, -1.0, 0.0));
    shapes.push(Box::new(sphere));
    let sphere = Sphere::new(Color::new(1.0, 0.0, 0.0), 100.0, Vertex4f::new(0.0, -100.5, -1.0, 0.0));
    shapes.push(Box::new(sphere));
    let shape_list = ShapeList::new(shapes);

    // Antialiasing - shoot multiple rays through the same pixel and average the colors
    let ns = 100;
    for x in 0..width {
        for y in 0..height {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let Open01(val) = random::<Open01<f64>>();
                let u = (x as f64 + val) / width as f64;
                let Open01(val) = random::<Open01<f64>>();
                let v = (y as f64 + val) / height as f64;
                let ray = camera.get_ray(u, v);
                let c = find_color(&ray, &shape_list);
                color += c;
            }
            color /= ns as f64;

            let index = ((y * width + x) * 4) as usize;
            pixel_data[index] = 0;   // A
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

fn find_color(ray: &Ray, shape_list: &ShapeList) -> Color {
    // t_min > 0, otherwise the rays get "stuck" and we overflow
    let intersection_points = shape_list.intersect(&ray, 0.001, f64::MAX);
    if intersection_points.is_empty() == false  {
        let hit = intersection_points[0];
        let target = hit.intersection_point.as_vector() + hit.normal + random_point_on_unit_sphere();
        let scattered_ray = Ray::new(hit.intersection_point, target - hit.intersection_point.as_vector());
        0.5 * find_color(&scattered_ray, shape_list)
    } else {
        let t = 0.5 * (ray.direction.y + 1.0);
        let color_vector = (1.0 - t) * Vector4f::new(1.0, 1.0, 1.0, 0.0) + t * Vector4f::new(0.5, 0.7, 1.0, 0.0);
        Color::new(color_vector.x, color_vector.y, color_vector.z)
    }
}

fn random_point_on_unit_sphere() -> Vector4f {
    let mut p;
    loop {
        let Open01(x) = random::<Open01<f64>>();
        let Open01(y) = random::<Open01<f64>>();
        let Open01(z) = random::<Open01<f64>>();
        // ensure vector is in range of (-1,1)
        p = 2.0 * Vector4f::new(x, y, z, 0.0) - Vector4f::new(1.0, 1.0, 1.0, 0.0);
        if p.norm() <= 1.0 {
            break;
        }
    }
    p
}
