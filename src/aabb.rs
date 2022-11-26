use crate::{ray::Ray, shapes::ShapeKind, vec3::Vec3};

#[derive(Debug, Copy, Clone)]

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn from_shape(shape: ShapeKind) -> Self {
        match shape {
            ShapeKind::Sphere(center, radius) => Self::new(
                center - Vec3::new(radius, radius, radius),
                center + Vec3::new(radius, radius, radius),
            ),
            ShapeKind::Plane(_) => todo!(),
        }
    }

    pub fn surrounding_box(&self, other: &Self) -> Self {
        let min = Vec3::new(
            f64::min(self.min.x, other.min.x),
            f64::min(self.min.y, other.min.y),
            f64::min(self.min.z, other.min.z),
        );
        let max = Vec3::new(
            f64::max(self.max.x, other.max.x),
            f64::max(self.max.y, other.max.y),
            f64::max(self.max.z, other.max.z),
        );

        Self::new(min, max)
    }

    // Reference: https://medium.com/@bromanz/another-view-on-the-classic-ray-aabb-intersection-algorithm-for-bvh-traversal-41125138b525
    pub fn hit(&self, ray: &Ray) -> bool {
        let inv_dir = ray.direction.inverse();

        let t0 = (self.min - ray.origin) * inv_dir;
        let t1 = (self.max - ray.origin) * inv_dir;

        let tsmall = Vec3::min(t0, t1);
        let tbig = Vec3::max(t0, t1);

        tsmall.max_component() < tbig.min_component() // Should this be < or <= ?
    }

    /* This is the original code that I was using, it's not the most optimal version, but it does show more clearly the logic behind this
    pub fn hit(&self, ray: &Ray) -> bool {
        // For each axis calculate the t0 and t1 that represent a interval
        let intervals = (0..3).map(|axis| {
            let t0 = f64::min(
                (self.min[axis] - ray.origin[axis]) / ray.direction[axis],
                (self.max[axis] - ray.origin[axis]) / ray.direction[axis],
            );
            let t1 = f64::max(
                (self.min[axis] - ray.origin[axis]) / ray.direction[axis],
                (self.max[axis] - ray.origin[axis]) / ray.direction[axis],
            );
            (t0, t1)
        });

        // Calculate the overlaping of the intervals
        let overlaping = intervals
        .reduce(|(l_min, l_max), (r_min, r_max)| {
                (f64::max(l_min, r_min), f64::min(l_max, r_max))
            })
            .unwrap();

            // If the t0 is less that the t1 we have an overlaping and therefore a hit
        return overlaping.0 < overlaping.1;
    }
    */
}
