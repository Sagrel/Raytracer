use serde::{Deserialize, Serialize};

use crate::ray::Ray;
use crate::scene::Scene;
use crate::{Real, Vector};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub lower_left_corner: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub origin: Vector,
}

impl Camera {
    pub fn new(scene: &Scene, aspect_ratio: Real) -> Camera {
        let theta = scene.fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (scene.look_from - scene.look_at).normalize();
        let u = Vector::Y.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lower_left_corner: scene.look_from - u * half_width - v * half_height - w,
            horizontal: u * 2.0 * half_width,
            vertical: v * 2.0 * half_height,
            origin: scene.look_from,
        }
    }

    pub fn get_pixel(&self, x_offset: Real, y_offset: Real) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * x_offset + self.vertical * y_offset
                - self.origin,
        )
    }
}
