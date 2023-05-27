use nanorand::*;
use rayon::{prelude::ParallelIterator, slice::ParallelSliceMut};

use crate::{bvh::Bvh, camera::Camera, config::Config, scene::Scene, Real, Vector};

fn indeces_2d(width: usize, height: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).flat_map(move |row| (0..width).map(move |col| (col, row)))
}

pub fn raytrace(config: &Config) -> impl IntoIterator<Item = Vector> {
    let scene = Scene::read_scene(&config.scene);
    let camera = Camera::new(&scene, config.aspect_ratio);
    let bvh = Bvh::new(&scene.shapes);

    let mut pixels = indeces_2d(config.width, config.height)
        .map(|index| (Vector::ZERO, index))
        .collect::<Vec<_>>();

    pixels
        .par_chunks_mut(config.width * config.chunk_size)
        .for_each(|chunk| {
            let mut rng = nanorand::tls_rng();
            for (pixel, (x, y)) in chunk.iter_mut() {
                for _ in 0..config.samples {
                    let x_offset = (*x as Real + rng.generate::<Real>()) / config.width as Real;
                    let y_offset = (*y as Real + rng.generate::<Real>()) / config.height as Real;
                    let ray = camera.get_pixel(x_offset, y_offset);

                    *pixel += ray.bounce(&bvh, &scene, &config.ambient_color, config.ttl);
                }
            }
        });

    pixels.into_iter().map(|(pixel, _)| pixel)
}
