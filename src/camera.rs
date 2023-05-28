use serde::{Deserialize, Serialize};

use crate::scene::Scene;
use crate::{ray::Ray, Matrix};
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
        Self::new_looking_at(scene.look_from, scene.look_at, scene.fov, aspect_ratio)
    }

    pub fn new_looking_at(
        origin: Vector,
        look_at: Vector,
        fov: Real,
        aspect_ratio: Real,
    ) -> Camera {
        let theta = fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (origin - look_at).normalize();
        let u = Vector::NEG_Y.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lower_left_corner: origin - u * half_width - v * half_height - w,
            horizontal: u * 2.0 * half_width,
            vertical: v * 2.0 * half_height,
            origin,
        }
    }

    pub fn new_angles(
        fov: Real,
        pitch: Real,
        yaw: Real,
        origin: Vector,
        aspect_ratio: Real,
    ) -> Self {
        let rotator = Matrix::from_euler(
            glam::EulerRot::default(),
            yaw.to_radians(),
            pitch.to_radians(),
            Real::to_radians(180.0),
        );

        Self::new_looking_at(origin, origin + rotator.mul_vec3(Vector::Z), fov, aspect_ratio)
    }

    pub fn get_pixel(&self, x_offset: Real, y_offset: Real) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * x_offset + self.vertical * y_offset
                - self.origin,
        )
    }
}
