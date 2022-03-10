use serde::{Serialize, Deserialize};

use crate::materials::Material;
use crate::ray::{Hit, Ray};
use crate::vec3::Vec3;

#[derive(Serialize, Deserialize, Debug)]
pub enum Shape {
    Sphere(Vec3, f64, Material),
    Plane([f64;5], Material)           
}

impl Shape {
    pub fn intersects(&self, ray: &Ray) -> Option<Hit> {
        match self {
            &Shape::Plane([x0,x1,y0,y1, k], mat) => {
                let t = (k-ray.origin.z) / ray.direction.z;
                let x = ray.origin.x + t*ray.direction.x;
                let y = ray.origin.y + t*ray.direction.y;
                if x < x0 || x > x1 || y < y0 || y > y1{
                    return None;
                }
                let t = t;
                let normal =  Vec3::new(0.0, 0.0, 1.0);
                let (normal, front_face) = if Vec3::dot(ray.direction, normal) < 0.0 {
                    (normal, true)
                } else {
                    (normal * -1.0, false)
                };
                Some(Hit::new(t, ray.point(t), normal, mat, front_face))
            }
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

                // We want the outwards facing normal
                let normal = (point - *center) / *radious;
                let (normal, front_face) = if Vec3::dot(ray.direction, normal) < 0.0 {
                    (normal, true)
                } else {
                    (normal * -1.0, false)
                };

                Some(Hit::new(root, point, normal, *mat, front_face))
            }
        }
    }
}
