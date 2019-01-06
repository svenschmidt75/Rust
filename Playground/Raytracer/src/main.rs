extern crate primitives;
extern crate rand;
extern crate sdl2;

use std::f64;

use rand::random;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;

use primitives::Camera::Camera;
use primitives::Color::Color;
use primitives::Dielectric::Dielectric;
use primitives::Lambertian::Lambertian;
use primitives::Metal::Metal;
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
    let lookat = Vertex4f::new(0.0, 0.0, -1.0, 0.0);
    let lookfrom = Vertex4f::new(-2.0, 2.0, 1.0, 0.0);
    let camera = Camera::new(lookfrom, lookat, Vector4f::new(0.0, 1.0, 0.0, 0.0), 40f32, width as f32 / height as f32, 2f32, (lookfrom - lookat).norm() as f32);

    // scene objects
    let shapes = create_scene1();
    let shape_list = ShapeList::new(shapes);

    // Antialiasing - shoot multiple rays through the same pixel and average the colors
    let ns = 1;
    for x in 0..width {
        for y in 0..height {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let val = random::<f64>();
                let u = (x as f64 + val) / width as f64;
                let val = random::<f64>();
                let v = (height as f64 - (y as f64 + val)) / height as f64;
                let ray = camera.get_ray(u, v);
                let c = find_color(ray, &shape_list, 1);
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

fn create_scene1() -> Vec<Box<Shape>> {
    let mut shapes = Vec::<Box<Shape>>::new();
    let sphere = Sphere::new(Color::new(1.0, 0.0, 0.0), 0.5, Vertex4f::new(0.0, 0.0, -1.0, 0.0), Box::new(Lambertian::new(Vector4f::new(0.1, 0.2, 0.5, 0.0))));
    shapes.push(Box::new(sphere));
    let sphere = Sphere::new(Color::new(0.0, 0.0, 0.0), 100.0, Vertex4f::new(0.0, -100.5, -1.0, 0.0), Box::new(Lambertian::new(Vector4f::new(0.8, 0.8, 0.0, 0.0))));
    shapes.push(Box::new(sphere));
    let sphere = Sphere::new(Color::new(1.0, 0.0, 0.0), 0.5, Vertex4f::new(1.0, 0.0, -1.0, 0.0), Box::new(Metal::new(Vector4f::new(0.8, 0.6, 0.2, 0.0), 0.3)));
    shapes.push(Box::new(sphere));
    let sphere = Sphere::new(Color::new(1.0, 0.0, 0.0), 0.5, Vertex4f::new(-1.0, 0.0, -1.0, 0.0), Box::new(Dielectric::new(1.5)));
    shapes.push(Box::new(sphere));
    let sphere = Sphere::new(Color::new(1.0, 0.0, 0.0), -0.45, Vertex4f::new(-1.0, 0.0, -1.0, 0.0), Box::new(Dielectric::new(1.5)));
    shapes.push(Box::new(sphere));
    return shapes;
}

fn find_color(ray: Ray, shape_list: &ShapeList, depth: u8) -> Color {
    // t_min > 0, otherwise the rays get "stuck" and we overflow
    let intersection_points = shape_list.intersect(&ray, 0.001, f64::MAX);
    if depth < 50 && intersection_points.is_empty() == false {
        let hit = &intersection_points[0];
        match hit.material.scatter(&ray, hit.intersection_point, hit.normal) {
            Some((scattered_ray, attenuation)) => {
                attenuation * find_color(scattered_ray, shape_list, depth + 1)
            },
            None => Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let t = 0.5 * (ray.direction.y + 1.0);
        let color_vector = (1.0 - t) * Vector4f::new(1.0, 1.0, 1.0, 0.0) + t * Vector4f::new(0.5, 0.7, 1.0, 0.0);
        Color::new(color_vector.x, color_vector.y, color_vector.z)
    }
}
