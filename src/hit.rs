use crate::{materials::MaterialRef, ray::Ray, Real, Vector};

#[derive(Debug, Copy, Clone)]
pub struct Hit {
    pub t: Real,
    normal: Vector,
    pub material: MaterialRef,
}

pub struct HitInfo {
    pub point: Vector,
    pub normal: Vector,
    pub front_face: bool,
    pub material: MaterialRef,
}

impl Hit {
    pub fn new(t: Real, normal: impl Into<Vector>, material: MaterialRef) -> Hit {
        Hit {
            t,
            normal: normal.into(),
            material,
        }
    }

    pub fn front_face(&self, ray: &Ray) -> bool {
        self.normal.dot(ray.direction).is_sign_negative()
    }

    pub fn get_hit_info(&self, ray: &Ray) -> HitInfo {
        let front_face = self.front_face(ray);
        let normal = if front_face {
            self.normal
        } else {
            -self.normal
        };
        HitInfo {
            point: ray.point(self.t),
            normal,
            material: self.material,
            front_face,
        }
    }
}
