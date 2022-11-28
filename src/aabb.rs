use crate::{ray::Ray, shapes::ShapeKind, Vector};

#[derive(Debug, Copy, Clone)]

pub struct AABB {
    pub min: Vector,
    pub max: Vector,
}

impl AABB {
    pub fn new(min: Vector, max: Vector) -> Self {
        Self { min, max }
    }

    pub fn from_shape(shape: ShapeKind) -> Self {
        match shape {
            ShapeKind::Sphere(center, radius) => Self::new(
                center - Vector::splat(radius),
                center + Vector::splat(radius),
            ),
            ShapeKind::Triangle(a, b, c) => Self::new(a.min(b).min(c), a.max(b).max(c)),
        }
    }

    pub fn surrounding_box(&self, other: &Self) -> Self {
        Self::new(self.min.min(other.min), self.max.max(other.max))
    }

    // Reference: https://medium.com/@bromanz/another-view-on-the-classic-ray-aabb-intersection-algorithm-for-bvh-traversal-41125138b525
    pub fn hit(&self, ray: &Ray) -> bool {
        let inv_dir = ray.direction.recip();

        let t0 = (self.min - ray.origin) * inv_dir;
        let t1 = (self.max - ray.origin) * inv_dir;

        let tsmall = t0.min(t1);
        let tbig = t0.max(t1);

        tsmall.max_element() <= tbig.min_element()
    }

    pub fn hit_fast(&self, ray_origin: Vector, ray_dir_recip: Vector) -> bool {
        let t0 = (self.min - ray_origin) * ray_dir_recip;
        let t1 = (self.max - ray_origin) * ray_dir_recip;

        let tsmall = t0.min(t1);
        let tbig = t0.max(t1);

        tsmall.max_element() <= tbig.min_element()
    }
}
