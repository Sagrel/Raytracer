use serde::{Deserialize, Serialize};

use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn new(
        look_from: impl Into<Vec3>,
        look_at: impl Into<Vec3>,
        up: impl Into<Vec3>,
        fov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let look_from = look_from.into();
        let look_at = look_at.into();
        let up = up.into();

        let theta = fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).normalized();
        let u = Vec3::cross(up, w).normalized();
        let v = Vec3::cross(w, u);

        Camera {
            lower_left_corner: look_from - u * half_width - v * half_height - w,
            horizontal: u * 2.0 * half_width,
            vertical: v * 2.0 * half_height,
            origin: look_from,
        }
    }

    pub fn get_pixel(&self, x_offset: f64, y_offset: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * x_offset + self.vertical * y_offset
                - self.origin,
        )
    }
}
