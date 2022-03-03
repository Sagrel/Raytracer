mod vec3;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

use image::ImageBuffer;
use rand::thread_rng;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use vec3::Vec3;

mod shapes;
use shapes::Shape;

mod camera;
use camera::Camera;

mod ray;

mod materials;
use materials::Material;

use rand::prelude::Rng;

const WIDHT: usize = 300;
const HEIGHT: usize = 300;
const SAMPLES: usize = 10;
const RATIO: f32 = WIDHT as f32 / HEIGHT as f32;
const TTL: i32 = 1024;

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
            let a = a as f32;
            let b = b as f32;
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * rng.gen::<f32>());

            let mut random_color = || Vec3::new(rng.gen(), rng.gen(), rng.gen());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_color() * random_color();
                    world.push(Shape::Sphere(center, 0.2, Material::Diffuse(albedo)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_color() * 0.5 + Vec3::new(0.5, 0.5, 0.5);
                    let fuzz = rng.gen::<f32>() * 0.5;
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

fn raytrace(world: &[Shape], camera: Camera, ambient_light: Vec3) -> Vec<Vec3> {
    optick::event!();

    let res = Arc::new(Mutex::new(vec![Vec3::zero(); HEIGHT * WIDHT]));

    (0..SAMPLES).into_par_iter().for_each(|i| {
        optick::register_thread(&format!("thread {}", i));
        let sample : Vec<_> = (0..(HEIGHT * WIDHT)).into_iter().map(|idx| {
            let mut rng = rand::thread_rng();
            let f = idx / WIDHT;
            let c = idx % WIDHT;

            let x_offset = (c as f32 + rng.gen::<f32>()) / WIDHT as f32;
            let y_offset = (f as f32 + rng.gen::<f32>()) / HEIGHT as f32;
            let ray = camera.get_pixel(x_offset, y_offset);

            ray.bounce(world, ambient_light, TTL)
        }).collect();
        
        let mut res = res.lock().unwrap();
        for (idx, pixel) in sample.iter().enumerate() {
            res[idx] = res[idx] + *pixel;
        }
    });

    // https://users.rust-lang.org/t/take-ownership-of-arc-mutex-t-inner-value/38097/2
    let mut res = Arc::try_unwrap(res).unwrap().into_inner().unwrap();

    for pixel in res.iter_mut() {
        *pixel = *pixel / SAMPLES as f32;
    }

    res
}

fn print_image(pixels: &[Vec3]) {
    let mut imgbuf = ImageBuffer::new(WIDHT as u32, HEIGHT as u32);

    for (c, f, pixel) in imgbuf.enumerate_pixels_mut() {
        let f = HEIGHT - 1 - f as usize;
        let color = pixels[c as usize + f * WIDHT] * 255.0;

        *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
    }

    imgbuf.save("salida.png").unwrap();
}

fn main() {
    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        RATIO,
    );
    let ambient_color = Vec3::new(0.5, 0.7, 1.0);

    println!(
        "Parametors: width = {} height = {} samples = {} ttl = {}",
        WIDHT, HEIGHT, SAMPLES, TTL
    );

    let now = Instant::now();
    let world = create_world();
    println!("Tiempo de creacion del mundo: {}s", now.elapsed().as_secs());

    optick::start_capture();

    let now = Instant::now();
    let pixels = raytrace(&world, camera, ambient_color);
    println!("Tiempo de raytracing: {}s", now.elapsed().as_secs());

    optick::stop_capture("raytracing_perf");

    let now = Instant::now();
    print_image(&pixels);
    println!("Tiempo de guardar la imagen: {}s", now.elapsed().as_secs());
}
