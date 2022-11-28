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
            ShapeKind::Triangle(_, _, _) => todo!(),
        }
    }
}
