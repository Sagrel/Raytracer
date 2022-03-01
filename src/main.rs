mod vec3;
use vec3::Vec3;

mod shapes;
use shapes::Shape;

mod camera;
use camera::Camera;

mod ray;

mod materials;
use materials::Material;

use rand::prelude::Rng;

use std::sync::Arc;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use std::time::Duration;

use show_image::{ImageInfo, KeyCode, make_window};



fn main()
{
    const WIDHT : usize = 1920;
    const HEIGHT : usize = 1080;
    const RATIO : f32 = WIDHT as f32 / HEIGHT as f32;   

    let mut pos = Vec3::new(-2.0, 2.0, 1.0);
    let mut pitch : f32 = 0.0;
    let mut yawn : f32 = 0.0;
    let up = Vec3::new(0.0, 1.0, 0.0);
    let fov = 90.0;

    
    
    let mut world : Vec<Shape>= Vec::new();

    let ambient_color = Vec3::new(0.5, 0.7, 1.0);

    world.push(Shape::Sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, Material::Diffuse(Vec3::new(0.8, 0.3, 0.3))));
    world.push(Shape::Sphere(Vec3::new(0.0, -100.5, -1.0), 100.0, Material::Diffuse(Vec3::new(0.8, 0.8, 0.0))));
    world.push(Shape::Sphere(Vec3::new(1.0, 0.0, -1.0), 0.5, Material::Metal(Vec3::new(0.8, 0.6, 0.2), 0.0)));
    world.push(Shape::Sphere(Vec3::new(-1.0, 0.0, -1.0), 0.5, Material::Dielectric(1.5)));
    world.push(Shape::Sphere(Vec3::new(-1.0, 0.0, -1.0), -0.45, Material::Dielectric(1.5)));

    let _ = Shape::Plane(Vec3::new(-1.0, 0.0, -10.0), -0.45, Material::Diffuse(Vec3::new(0.8, 0.3, 0.3)));

    
    let pool = ThreadPool::new(12);
    let shared_world = Arc::new(world);


    
    // Create a window and display the image.
    let window = make_window("image").unwrap();
    
    let mut image = vec![Vec3::zero(); WIDHT * HEIGHT];

    let mut samples = 1.0;
    let mut reiniciar = true;

    while let Ok(event) = window.wait_key(Duration::from_millis(20)) 
    {      
        let direction = Vec3::new(0.0,0.0,-1.0);

        let forward = direction.rotate_x(pitch).rotate_y(-yawn);
        let right = Vec3::cross(forward, up).normalized() * -1.0;
        let camera = Camera::new(pos, pos + forward, up, fov, RATIO);
        if let Some(event) = event 
        {
            reiniciar = true;
            if event.key == KeyCode::Escape 
            {
                break;
            }
            if event.key == KeyCode::Character("W".to_string())
            {
                samples = 1.0;
                pos = pos + forward;
            }
            if event.key == KeyCode::Character("S".to_string())
            {
                samples = 1.0;
                pos = pos - forward;
            }
            if event.key == KeyCode::Character("A".to_string())
            {
                samples = 1.0;
                pos = pos - right;
            }
            if event.key == KeyCode::Character("D".to_string())
            {
                samples = 1.0;
                pos = pos + right;
            }
            if event.key == KeyCode::ArrowUp
            {
                samples = 1.0;
                pitch += 0.2;
            }
            if event.key == KeyCode::ArrowDown
            {
                samples = 1.0;
                pitch -= 0.2;
            }
            if event.key == KeyCode::ArrowLeft
            {
                samples = 1.0;
                yawn += 0.2;
            }
            if event.key == KeyCode::ArrowRight
            {
                samples = 1.0;
                yawn -= 0.2;
            }
        }

        let (tx, rx) = channel();

        for f in 0..HEIGHT 
        {
    
            let local_world = shared_world.clone();
            let child_tx = tx.clone();
    
            pool.execute(move || 
            {
                let mut rng = rand::thread_rng();
                let mut fila = vec![Vec3::zero(); WIDHT];
                for c in 0..WIDHT 
                {    
                    let x_offset = (c as f32 + rng.gen::<f32>()) / WIDHT as f32;
                    let y_offset = (f as f32 + rng.gen::<f32>()) / HEIGHT as f32;
                    let ray = camera.get_pixel(x_offset, y_offset);         
    
                    fila[c] = ray.bounce(&local_world, ambient_color, 1024);          
                }
                child_tx.send((f, fila)).unwrap();
            });
        }
    
        drop(tx);    
            
        for (f, color) in rx.iter()
        {
            for c in 0..WIDHT 
            {
                if reiniciar
                {
                    image[c + f * WIDHT] = color[c];    // TODO Esto deberia reiniciar la imagen cada vez que nos movemos pero no lo hace
                }
                else
                {
                    image[c + f * WIDHT] = (image[c + f * WIDHT] * samples + color[c]) / (samples + 1.0);
                }
            }        
        }
        reiniciar = false;
        samples += 1.0;
    
        let mut pixeles = vec![255; WIDHT * HEIGHT * 3];
    
        let mut i = WIDHT * HEIGHT * 3 - 3;
        for color in image.iter()
        {
            pixeles[i] = (color.x * 255.0) as u8;
            pixeles[i+1] = (color.y * 255.0) as u8;
            pixeles[i+2] = (color.z * 255.0) as u8;
            i -= 3
        }
    
    
        let picture = (pixeles, ImageInfo::rgb8(WIDHT, HEIGHT));
        
        window.set_image(&picture, "image-001").unwrap();
    }

    show_image::stop().unwrap();

}
