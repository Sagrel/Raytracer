mod vec3;
use vec3::Vec3;

mod shapes;
use shapes::Shape;

mod camera;
use camera::Camera;

mod ray;

mod materials;
use materials::Material;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use rand::prelude::Rng;

use std::sync::Arc;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use std::time::Instant;

fn main() -> std::io::Result<()>
{
    const WIDHT : usize = 1920;
    const HEIGHT : usize = 1080;
    const SAMPLES : usize = 50;
    const RATIO : f32 = WIDHT as f32 / HEIGHT as f32;

    let mut image = vec![Vec3::zero(); WIDHT * HEIGHT];

    let camera = Camera::new(Vec3::new(-2.0, 2.0, 1.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, RATIO);
    
    let mut world : Vec<Shape>= Vec::new();

    let ambient_color = Vec3::new(0.5, 0.7, 1.0);

    world.push(Shape::Sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, Material::Diffuse(Vec3::new(0.8, 0.3, 0.3))));
    world.push(Shape::Sphere(Vec3::new(0.0, -100.5, -1.0), 100.0, Material::Diffuse(Vec3::new(0.8, 0.8, 0.0))));
    world.push(Shape::Sphere(Vec3::new(1.0, 0.0, -1.0), 0.5, Material::Metal(Vec3::new(0.8, 0.6, 0.2), 0.0)));
    world.push(Shape::Sphere(Vec3::new(-1.0, 0.0, -1.0), 0.5, Material::Dielectric(1.5)));
    world.push(Shape::Sphere(Vec3::new(-1.0, 0.0, -1.0), -0.45, Material::Dielectric(1.5)));

    let _ = Shape::Plane(Vec3::new(-1.0, 0.0, -10.0), -0.45, Material::Diffuse(Vec3::new(0.8, 0.3, 0.3)));

    let now = Instant::now();
    
    
    let pool = ThreadPool::new(12);
    let shared_world = Arc::new(world);

    let (tx, rx) = channel();

    for f in 0..HEIGHT {

        let local_world = shared_world.clone();
        let child_tx = tx.clone();

        pool.execute(move || 
        {
            let mut rng = rand::thread_rng();
            let mut fila = vec![Vec3::zero(); WIDHT];
            for c in 0..WIDHT 
            {
                let mut color = Vec3::zero();

                for _ in 0..SAMPLES 
                {
                    let x_offset = (c as f32 + rng.gen::<f32>()) / WIDHT as f32;
                    let y_offset = (f as f32 + rng.gen::<f32>()) / HEIGHT as f32;
                    let ray = camera.get_pixel(x_offset, y_offset);

                    color = color + ray.bounce(&local_world, ambient_color, 1024); 
                }            

                fila[c] = color / SAMPLES as f32;               
            }
            child_tx.send((f, fila)).unwrap();
        });
    }

    drop(tx);

    let progres_per_line = 1.0 / HEIGHT as f32;
    let mut progres = 0.0;
    for (f, color) in rx.iter()
    {
        for c in 0..WIDHT 
        {
            image[c + f * WIDHT] = color[c];
        }        

        if progres == 0.0 
        {
            print!("0%\n");   
        }
        else  if progres < 0.25 && progres + progres_per_line >= 0.25
        {
            print!("25%\n");   
        }
        else  if progres < 0.5 && progres + progres_per_line >= 0.5
        {
            print!("50%\n");   
        }
        else  if progres < 0.75 && progres + progres_per_line >= 0.75
        {
            print!("75%\n");   
        }
        progres += progres_per_line;
    }

    print!("Done!\n");  

    print!("Time spent on raytracing {}ms\n", now.elapsed().as_millis());

    let printing = Instant::now();

    let f = File::create("image.ppm")?;
    let mut file = BufWriter::new(f);

    file.write_all(format!("P3\n{} {}\n255\n", WIDHT, HEIGHT).as_bytes())?;

    for notf in 0..HEIGHT
    {
        let f = HEIGHT - notf - 1;
        for c in 0..WIDHT 
        {
            let color = image[c + f * WIDHT] * 255.0;
            file.write_all(format!("{} {} {}\n", color.x.round(), color.y.round(), color.z.round()).as_bytes())?;
        }        
    }

    print!("Time spent on printing {}ms\n", printing.elapsed().as_millis());

    file.flush()
}
