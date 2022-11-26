use std::fs::File;

use fastrand::Rng;
use rayon::{prelude::ParallelIterator, slice::ParallelSliceMut};

use crate::{bvh::BVH, config::Config, ray::Ray, vec3::Vec3, world::World};

fn indeces_2d(width: usize, height: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).flat_map(move |row| (0..width).map(move |col| (col, row)))
}

fn read_scene(scene: &str) -> World {
    serde_json::from_reader(File::open(format!("./scenes/{scene}.json")).unwrap()).unwrap()
}

/// This version is just a less vervose `raytrace_par_chunks` but maybe less performant
pub fn raytrace(config: &Config) -> impl IntoIterator<Item = Vec3> {
    let world = read_scene(&config.scene);
    let bvh = BVH::new(&world.shapes);

    let get_hit = |ray: &Ray| bvh.hit(ray, &world.shapes);

    let mut pixels = indeces_2d(config.width, config.height)
        .map(|index| (Vec3::zero(), index))
        .collect::<Vec<_>>();

    pixels
        .par_chunks_mut(config.width * config.chunk_size)
        .for_each(|chunk| {
            let rng = Rng::new();
            for (pixel, (x, y)) in chunk.iter_mut() {
                for _ in 0..config.samples {
                    let x_offset = (*x as f64 + rng.f64()) / config.width as f64;
                    let y_offset = (*y as f64 + rng.f64()) / config.height as f64;
                    let ray = config.camera.get_pixel(x_offset, y_offset);

                    *pixel =
                        *pixel + ray.bounce(&get_hit, &world, &config.ambient_color, config.ttl);
                }
                *pixel = *pixel / config.samples as f64; // TODO Remove this and do gamma correction in image printing
            }
        });

    pixels.into_iter().map(|(pixel, _)| pixel)
}
