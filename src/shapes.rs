use serde::{Deserialize, Serialize};

use crate::hit::Hit;
use crate::materials::MaterialRef;
use crate::ray::Ray;
use crate::{Real, Vector};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ShapeKind {
    Sphere(Vector, Real),
    Triangle(Vector, Vector, Vector),
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Shape {
    pub kind: ShapeKind,
    pub material: MaterialRef,
}

impl Shape {
    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        match self.kind {
            ShapeKind::Sphere(center, radious) => {
                let o_c = ray.origin - center;
                let a = ray.direction.dot(ray.direction);
                let half_b = ray.direction.dot(o_c);
                let c = o_c.dot(o_c) - radious * radious;

                let discriminant = half_b * half_b - a * c;

                if discriminant.is_sign_negative() {
                    return None;
                }

                let squared_discriminant = discriminant.sqrt();

                let mut root = (-half_b - squared_discriminant) / a;

                // If it's too close to the camera don't take it
                if root < Real::MIN_POSITIVE {
                    root = (-half_b + squared_discriminant) / a;
                    if root < Real::MIN_POSITIVE {
                        return None;
                    }
                }

                let point = ray.point(root);
                let normal = (point - center) / radious;

                Some(Hit::new(root, normal, self.material))
            }
            ShapeKind::Triangle(a, b, c) => {
                // Code stolen from https://docs.rs/bvh/latest/src/bvh/ray.rs.html#289-340
                // It implements the algorithim https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
                let a_to_b = b - a;
                let a_to_c = c - a;

                // Begin calculating determinant - also used to calculate u parameter
                // u_vec lies in view plane
                // length of a_to_c in view_plane = |u_vec| = |a_to_c|*sin(a_to_c, dir)
                let u_vec = ray.direction.cross(a_to_c);

                // If determinant is near zero, ray lies in plane of triangle
                // The determinant corresponds to the parallelepiped volume:
                // det = 0 => [dir, a_to_b, a_to_c] not linearly independant
                let det = a_to_b.dot(u_vec);

                // Only testing positive bound, thus enabling backface culling
                // If backface culling is not desired write:
                // det < EPSILON && det > -EPSILON
                if det < Real::EPSILON {
                    return None;
                }

                let inv_det = 1.0 / det;

                // Vector from point a to ray origin
                let a_to_origin = ray.origin - a;

                // Calculate u parameter
                let u = a_to_origin.dot(u_vec) * inv_det;

                // Test bounds: u < 0 || u > 1 => outside of triangle
                if !(0.0..=1.0).contains(&u) {
                    return None;
                }

                // Prepare to test v parameter
                let v_vec = a_to_origin.cross(a_to_b);

                // Calculate v parameter and test bound
                let v = ray.direction.dot(v_vec) * inv_det;
                // The intersection lies outside of the triangle
                if v < 0.0 || u + v > 1.0 {
                    return None;
                }

                let dist = a_to_c.dot(v_vec) * inv_det;

                if dist > Real::EPSILON {
                    Some(Hit::new(dist, a_to_b.cross(a_to_c), self.material))
                } else {
                    None
                }
            }
        }
    }
}
