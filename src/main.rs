mod vector;
mod ray;
mod sphere;
mod color;
mod material;
mod hit;

use crate::vector::{ Vector3 };
use crate::ray::{ Ray, Intersectable };
use crate::sphere::{ Sphere };
use crate::color::{ Color };
use crate::hit::{ Hit };
use crate::material::{ Material };

use std::fs::File;
use std::io::Write;

use rand::{ thread_rng, prelude::RngCore, Rng };

fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::std::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }
}

#[repr(C, packed(1))]
struct BMPHeader {
    file_type: [u8; 2],
    file_size: u32,
    reserved1: u16,
    reserved2: u16,
    bitmap_offset: u32,
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    size_of_bitmap: u32,
    horizontal_resolution: i32,
    vertical_resolution: i32,
    colors_used: u32,
    colors_important: u32,
    red_mask: [u8; 4],
    green_mask: [u8; 4],
    blue_mask: [u8; 4],
}

fn get_closest_hit(ray: &Ray, objects: &Vec::<Box<dyn Intersectable>>) -> Option<Hit> {
    let mut nearest_hit: Option<Hit> = Option::None;

    for object in objects {
        match object.intersect(ray) {
            Some(hit) => {
                match nearest_hit {
                    Some(nearest) => {
                        if hit.distance < nearest.distance {
                            nearest_hit = Option::from(hit);
                        }
                    },
                    None => {
                        nearest_hit = Option::from(hit);
                    }
                }
            },
            None => {}
        }
    }

    return nearest_hit;
}

fn get_ray_color(ray: &Ray, objects: &Vec::<Box<dyn Intersectable>>, rng: &mut dyn RngCore, depth: u64) -> Color {
    match get_closest_hit(&ray, &objects) {
        Some(hit) => {
            let mut color = hit.material.diffuse;

            if depth > 5 {
                let rand: f64 = rng.gen();
                let max_component = color.r.max(color.g.max(color.b));
                if rand < max_component && depth < 500 {
                    color = color * (1.0 / max_component);
                } else {
                    return hit.material.emission;
                }
            }

            let r1 = rng.gen::<f64>() * std::f64::consts::TAU;

            let r2 = rng.gen::<f64>();
            let r2s = r2.sqrt();

            let w = hit.normal;

            let tmp = if w.x.abs() > 0.1 {
                Vector3::new(0.0, 1.0, 0.0)
            } else {
                Vector3::new(1.0, 0.0, 0.0)
            };
            let u = Vector3::cross(&tmp, &w);

            let v = Vector3::cross(&w, &u);

            let new_dir = u * r1.cos() * r2s + v * r1.sin() * r2s + w * (1.0 - r2).sqrt();
            let new_ray = Ray::new(hit.position, new_dir.normalized());

            return hit.material.emission + color * get_ray_color(&new_ray, objects, rng, depth + 1);
        },
        None => {
            return Color::new(0.0, 0.0, 0.0);
        },
    }
}

use std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let scene_path;
    if args.len() == 1 {
        scene_path = "test.scene";
    } else if args.len() > 2 {
        panic!("Too many arguments!");
    } else {
        scene_path = &args[1];
    }

    let mut scene_file = File::open(scene_path).expect("Unable to open scene file!");
    let mut scene_source = String::new();
    scene_file.read_to_string(&mut scene_source).unwrap();

    let mut tokens = Vec::<&str>::new();
    for line in scene_source.lines() {
        for token in line.split_whitespace() {
            tokens.push(token);
        }
    }

    let mut token_index: usize = 0;
    
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut num_samples: usize = 1;
    let mut camera_position = Vector3::new(0.0, 0.0, 0.0);
    let mut camera_forward = Vector3::new(0.0, 0.0, 1.0).normalized();
    let mut objects = Vec::<Box<dyn Intersectable>>::new();

    loop {
        if token_index >= tokens.len() {
            break;
        }

        let mut next = || {
            let token = &tokens[token_index];
            token_index += 1;
            return token;
        };

        let name = next();
        match name {
            &"width" => {
                assert_eq!(next(), &"=");
                width = next().parse::<usize>().unwrap();
            },
            &"height" => {
                assert_eq!(next(), &"=");
                height = next().parse::<usize>().unwrap();
            },
            &"samples" => {
                assert_eq!(next(), &"=");
                num_samples = next().parse::<usize>().unwrap();
            },
            &"camera_position" => {
                assert_eq!(next(), &"=");
                assert_eq!(next(), &"{");
                    let mut position = Vector3::new(0.0, 0.0, 0.0);
                    position.x = next().parse::<f64>().unwrap();
                    position.y = next().parse::<f64>().unwrap();
                    position.z = next().parse::<f64>().unwrap();
                assert_eq!(next(), &"}");
                camera_position = position;
            },
            &"camera_direction" => {
                assert_eq!(next(), &"=");
                assert_eq!(next(), &"{");
                    let mut direction = Vector3::new(0.0, 0.0, 0.0);
                    direction.x = next().parse::<f64>().unwrap();
                    direction.y = next().parse::<f64>().unwrap();
                    direction.z = next().parse::<f64>().unwrap();
                assert_eq!(next(), &"}");
                camera_forward = direction.normalized();
            },
            &"Sphere" => {
                assert_eq!(next(), &"{");
                    assert_eq!(next(), &"{");
                        let mut position = Vector3::new(0.0, 0.0, 0.0);
                        position.x = next().parse::<f64>().unwrap();
                        position.y = next().parse::<f64>().unwrap();
                        position.z = next().parse::<f64>().unwrap();
                    assert_eq!(next(), &"}");
                        let radius = next().parse::<f64>().unwrap();
                    assert_eq!(next(), &"{");
                        let mut diffuse = Color::new(0.0, 0.0, 0.0);
                        diffuse.r = next().parse::<f64>().unwrap();
                        diffuse.g = next().parse::<f64>().unwrap();
                        diffuse.b = next().parse::<f64>().unwrap();
                    assert_eq!(next(), &"}");
                    assert_eq!(next(), &"{");
                        let mut emission = Color::new(0.0, 0.0, 0.0);
                        emission.r = next().parse::<f64>().unwrap();
                        emission.g = next().parse::<f64>().unwrap();
                        emission.b = next().parse::<f64>().unwrap();
                    assert_eq!(next(), &"}");
                assert_eq!(next(), &"}");

                objects.push(Box::from(Sphere::new(position, radius, Material::new(diffuse, emission))));
            },
            _ => panic!("Unknown name!"),
        }
    }

    // NOTE: Open file here so we know that we will be able to output image after its been generated
    let mut file = File::create("./out_image.bmp").expect("Unable to create file!");

    /*
    let blue_material = Material::new(
        Color::new(0.2, 0.4, 0.8),
        Color::new(0.0, 0.0, 0.0),
    );

    let light_material = Material::new(
        Color::new(1.0, 1.0, 1.0),
        Color::new(2.5, 2.5, 2.5),
    );

    objects.push(Box::from(Sphere::new(Vector3::new(2.0, 0.0, 0.0), 2.0, blue_material)));
    objects.push(Box::from(Sphere::new(Vector3::new(-2.0, 0.0, -1.0), 1.0, light_material)));
    */

    let camera_right = Vector3::cross(&Vector3::new(0.0, 1.0, 0.0), &camera_forward).normalized();
    let camera_up = Vector3::cross(&camera_forward, &camera_right).normalized();

    let mut pixels = vec![Color::new(0.0, 0.0, 0.0); (width * height) as usize];

    // TODO: Seeded per pixel
    let mut rng = thread_rng();

    let aspect = width as f64 / height as f64;

    let mut i: usize = 0;
    for y in 0..height {
        let norm_y = (y as f64 / height as f64) * 2.0 - 1.0;

        for x in 0..width {
            let norm_x = (x as f64 / width as f64) * 2.0 - 1.0;

            let ray = Ray::new(
                camera_position,
                (camera_forward
                + (camera_right * (norm_x * aspect))
                + (camera_up * norm_y)).normalized()
            );

            let pixel = &mut pixels[(x + y * width) as usize];

            for _ in 0..num_samples {
                *pixel = *pixel + get_ray_color(&ray, &objects, &mut rng, 0) * (1.0 / num_samples as f64);
            }

            i += 1;
            if i % 50 == 0 {
                print!("Rendering: {:.2}% ({} of {})\r", (i as f64 / (width * height) as f64) * 100.0, i, width * height);
            }
        }
    }
    println!("Rendering: 100.00%");

    let header = BMPHeader {
        file_type: [0x42, 0x4D],
        file_size: (std::mem::size_of::<BMPHeader>() + (width * height * 4)) as u32,
        reserved1: 0,
        reserved2: 0,
        bitmap_offset: std::mem::size_of::<BMPHeader>() as u32,
        size: 40,
        width: width as i32,
        height: height as i32,
        planes: 1,
        bits_per_pixel: 32,
        compression: 3,
        size_of_bitmap: 0,
        horizontal_resolution: 0,
        vertical_resolution: 0,
        colors_used: 0,
        colors_important: 0,
        red_mask:   [0xFF, 0x00, 0x00, 0x00],
        green_mask: [0x00, 0xFF, 0x00, 0x00],
        blue_mask:  [0x00, 0x00, 0xFF, 0x00],
    };

    file.write_all(any_as_u8_slice(&header)).expect("Unable to write header to file!");

    i = 0;
    for pixel in pixels {
        file.write_all(&[
            (pixel.r * 255.0) as u8,
            (pixel.g * 255.0) as u8,
            (pixel.b * 255.0) as u8,
            0 as u8,
        ]).expect("Unable to write to file!");

        i += 1;
        if i % 500 == 0 {
            print!("Ouputing: {:.2}% ({} of {})\r", (i as f64 / (width * height) as f64) * 100.0, i, width * height);
        }
    }
    println!("Ouputing: 100.00%");

    println!("Done.");
}
