mod vec3;
use std::fs::File;
use std::time::Instant;

use image::imageops::flip_vertical_in_place;
use image::ImageBuffer;
use time::OffsetDateTime;
use vec3::Vec3;

mod hit;
mod world;

mod shapes;

mod camera;

mod materials;
mod ray;
mod scene_gerenators;
use raytrace::*;

use crate::config::Config;

mod raytrace;

mod aabb;
mod bvh;
mod config;

fn main() {
    let config: Config = match File::open("config.json") {
        Ok(file) => serde_json::from_reader(file).unwrap(),
        Err(_) => Default::default(),
    };

    println!(
        "Parameters: width = {} height = {} samples = {} ttl = {} chunk size = {}",
        config.width, config.height, config.samples, config.ttl, config.chunk_size
    );

    let now = Instant::now();
    let pixels = raytrace(&config);

    let rays = config.width * config.height * config.samples;
    let millis = now.elapsed().as_millis();
    let rays_sec = rays as f64 / (millis as f64 / 1000.0);

    println!("Time: {millis}ms  Rays per second: {}", rays_sec.floor());

    print_image(pixels, &config);
}

fn print_image(pixels: impl IntoIterator<Item = Vec3>, config: &Config) {
    let mut imgbuf = ImageBuffer::new(config.width as u32, config.height as u32);
    for (img_pixel, calculated_pixel) in imgbuf.pixels_mut().zip(pixels) {
        let color = (calculated_pixel / config.samples as f64) * 255.0;
        *img_pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
    }

    flip_vertical_in_place(&mut imgbuf);

    imgbuf
        .save(format!(
            "./results/{}_{}.png",
            config.scene,
            OffsetDateTime::now_utc()
        ))
        .unwrap();
}
