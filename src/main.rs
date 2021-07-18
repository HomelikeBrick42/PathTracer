mod vector;
mod ray;
mod sphere;
mod color;

use crate::vector::{ Vector3 };
use crate::ray::{ Ray, Intersectable };
use crate::sphere::{ Sphere };
use crate::color::{ Color };

use std::fs::File;
use std::io::Write;

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
    file_type: u16,
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
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
}

fn main() {
    let sphere = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.5);

    let width: u32 = 1280;
    let height: u32 = 720;

    let camera_pos = Vector3::new(0.0, 0.0, -4.0);
    let camera_up = Vector3::new(0.0, 1.0, 0.0).normalized();
    let camera_right = Vector3::new(1.0, 0.0, 0.0).normalized();
    let camera_forward = Vector3::new(0.0, 0.0, 1.0).normalized();

    let clear_color = Color::new(0.1, 0.1, 0.1, 1.0);
    let mut pixels = vec![clear_color; (width * height) as usize];

    let aspect = width as f64 / height as f64;
    for x in 0..width {
        for y in 0..height {
            let norm_x = (x as f64 / width as f64) * 2.0 - 1.0;
            let norm_y = (y as f64 / height as f64) * 2.0 - 1.0;

            let ray = Ray::new(
                camera_pos,
                (camera_forward
                + (camera_right * norm_x * aspect)
                + (camera_up * norm_y)).normalized()
            );

            match sphere.intersect(&ray) {
                Some((normal, _color)) => {
                    pixels[(x + y * width) as usize] = Color::new(normal.x * 0.5 + 0.5, normal.y * 0.5 + 0.5, normal.z * 0.5 + 0.5, 1.0);
                },
                _ => {},
            }
        }
    }

    let header = BMPHeader {
        file_type: 0x4D42,
        file_size: std::mem::size_of::<BMPHeader>() as u32 + (width * height * 4),
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
        red_mask:   0x000000FF,
        green_mask: 0x0000FF00,
        blue_mask:  0x00FF0000,
    };

    let mut file = File::create("./out_image.bmp").expect("Unable to create file!");
    file.write_all(any_as_u8_slice(&header)).expect("Unable to write header to file!");

    for pixel in pixels {
        file.write_all(&[
            (pixel.r * 255.0) as u8,
            (pixel.g * 255.0) as u8,
            (pixel.b * 255.0) as u8,
            (pixel.a * 255.0) as u8,
        ]).expect("Unable to write to file!");
    }
}
