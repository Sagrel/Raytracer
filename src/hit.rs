use crate::{materials::MaterialRef, vec3::Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Hit {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: MaterialRef,
    pub front_face: bool,
}

impl Hit {
    pub fn new(
        t: f64,
        point: impl Into<Vec3>,
        normal: impl Into<Vec3>,
        material: MaterialRef,
        front_face: bool,
    ) -> Hit {
        Hit {
            t,
            point: point.into(),
            normal: normal.into(),
            material,
            front_face,
        }
    }
}
