use std::mem::discriminant;

use crate::materials::Material;
use crate::ray::{Hit, Ray};
use crate::vec3::Vec3;

pub enum Shape {
    Sphere(Vec3, f64, Material),
    Plane(Vec3, f64, Material),
}

impl Shape {
    pub fn intersects(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Shape::Sphere(center, radious, mat) => {
                let o_c = ray.origin - *center;
                let a = Vec3::dot(ray.direction, ray.direction);
                let half_b = Vec3::dot(ray.direction, o_c);
                let c = Vec3::dot(o_c, o_c) - radious * radious;

                let discriminant = half_b * half_b - a * c;

                if discriminant < 0.0 {
                    return None;
                }

                let squared_discriminant = discriminant.sqrt();

                let mut root = (-half_b - squared_discriminant) / a;

                let min_t = 0.001;

                // If it's too close to the camera don't take it
                if root < min_t {
                    root = (-half_b + squared_discriminant) / a;
                    if root < min_t {
                        return None;
                    }
                }

                let point = ray.point(root);

                Some(Hit::new(root, point, (point - *center) / *radious, *mat))
            }
            Shape::Plane(normal, distance, mat) => {
                let d = Vec3::dot(*normal, ray.direction);
                let pos = (*normal) * (*distance);

                if d.abs() < 0.05 {
                    return None;
                }

                let t = Vec3::dot(pos - ray.origin, *normal) / d;

                if t < 0.0 {
                    return None;
                }

                Some(Hit::new(t, ray.point(t), *normal, *mat))
            }
        }
    }
}
