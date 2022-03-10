mod vec3;
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use image::ImageBuffer;
use indicatif::ProgressBar;
use rand::thread_rng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;
use serde::Deserialize;
use serde::Serialize;
use vec3::Vec3;

mod shapes;
use shapes::Shape;

mod camera;
use camera::Camera;

mod ray;

mod materials;
use materials::Material;

use rand::prelude::Rng;

fn create_world() -> Vec<Shape> {
    let mut world: Vec<Shape> = Vec::new();

    let ground_material = Material::Diffuse(Vec3::new(0.5, 0.5, 0.5));
    world.push(Shape::Sphere(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());

            let mut random_color = || Vec3::new(rng.gen(), rng.gen(), rng.gen());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_color() * random_color();
                    world.push(Shape::Sphere(center, 0.2, Material::Diffuse(albedo)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_color() * 0.5 + Vec3::new(0.5, 0.5, 0.5);
                    let fuzz = rng.gen::<f64>() * 0.5;
                    world.push(Shape::Sphere(center, 0.2, Material::Metal(albedo, fuzz)));
                } else {
                    // glass
                    world.push(Shape::Sphere(center, 0.2, Material::Dielectric(1.5)));
                }
            }
        }
    }

    world.push(Shape::Sphere(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(1.5),
    ));
    world.push(Shape::Sphere(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Diffuse(Vec3::new(0.4, 0.2, 0.1)),
    ));
    world.push(Shape::Sphere(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    ));

    world
}

fn raytrace(config: &Config) -> Vec<Vec3> {
    let pb = ProgressBar::new((config.height * config.width) as u64);

    let mut pixels = vec![Vec3::zero(); config.height * config.width];
    //optick::event!();
    pixels
        .par_chunks_mut(config.width * config.chunk_size) // Divide work in chunks
        .enumerate() // Enumerate the chunks to get back the pixel coordinates
        .for_each(|(idx, chunk)| {
            // This is done in parallel
            //optick::register_thread("");
            let mut rng = rand::thread_rng();
            for (pos, pixel) in chunk.iter_mut().enumerate() {
                //optick::event!();
                // For each pixel in the chunck
                // We calculate the current pixel position
                let x = (pos % config.width) as f64;
                let y = (idx * config.chunk_size + pos / config.width) as f64;
                // And finally we sample the pixel config.samples amoults of times and average them
                // TODO should we divide by samples every time to avoid going over the max f64? Probably not
                for _ in 0..config.samples {
                    let x_offset = (x + rng.gen::<f64>()) / config.width as f64;
                    let y_offset = (y + rng.gen::<f64>()) / config.height as f64;
                    let ray = config.camera.get_pixel(x_offset, y_offset);

                    *pixel = *pixel + ray.bounce(&config.world, &config.ambient_color, config.ttl);
                }
                *pixel = *pixel / config.samples as f64;

                //pb.inc(1);
            }
            pb.inc((config.width * config.chunk_size) as u64);
        });

    pb.finish_and_clear();

    pixels
}

fn print_image(pixels: &[Vec3], config: &Config) {
    let mut imgbuf = ImageBuffer::new(config.width as u32, config.height as u32);

    for (c, f, pixel) in imgbuf.enumerate_pixels_mut() {
        let f = config.height - 1 - f as usize;
        let color = pixels[c as usize + f * config.width] * 255.0;

        *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
    }

    imgbuf.save("salida.png").unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    camera: Camera,
    world: Vec<Shape>,
    ambient_color: Vec3,
    width: usize,
    height: usize,
    samples: usize,
    ttl: i32,
    chunk_size: usize,
}

// TODO Cornell box

fn main() {
    let config: Config = if let Some(path) = env::args().nth(1) {
        match File::open(&path) {
            Ok(file) => serde_json::from_reader(file)
                .unwrap_or_else(|_| panic!("We could not read the file {}", path)),
            Err(_) => {
                eprintln!("The file '{}' does not exist", path);
                return;
            }
        }
    } else {
        let camera = Camera::new(
            Vec3::new(-2.0, 2.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            RATIO,
        );
        let ambient_color = Vec3::new(0.5, 0.7, 1.0);
        const WIDHT: usize = 1366;
        const HEIGHT: usize = 768;
        const SAMPLES: usize = 10;
        const CHUNK_ROWS: usize = 2;
        const RATIO: f64 = WIDHT as f64 / HEIGHT as f64;
        const TTL: i32 = 1024;
        let world = create_world();

        let c = Config {
            camera,
            ambient_color,
            world,
            width: WIDHT,
            height: HEIGHT,
            samples: SAMPLES,
            chunk_size: CHUNK_ROWS,
            ttl: TTL,
        };
        let json = serde_json::to_string_pretty(&c).expect("Could not serialize world");
        File::create("basic.json")
            .unwrap()
            .write_all(json.as_bytes())
            .expect("Could not write file");
        c
    };

    println!(
        "Parameters: width = {} height = {} samples = {} ttl = {} chunk size = {}",
        config.width, config.height, config.samples, config.ttl, config.chunk_size
    );

    //optick::start_capture();
    let now = Instant::now();
    let pixels = raytrace(&config);
    println!("Tiempo de raytracing: {}s", now.elapsed().as_secs());
    //optick::stop_capture("raytracing_perf");

    print_image(&pixels, &config);
}
