use nanorand::Rng;

use crate::{aabb::AABB, hit::Hit, ray::Ray, shapes::Shape, Vector};

type ShapeRef = usize;

// It's basically a binary tree
pub enum BVHKind {
    Node(Box<BVH>, Box<BVH>),
    Leaf(ShapeRef),
}

pub struct BVH {
    pub aabb: AABB,
    pub kind: BVHKind,
}

impl BVH {
    pub fn new(shapes: &[Shape]) -> BVH {
        let mut aabbs: Vec<(ShapeRef, AABB)> = shapes
            .iter()
            .map(|shape| AABB::from_shape(shape.kind))
            .enumerate()
            .collect();

        *Self::create_bvh(&mut aabbs)
    }

    pub fn hit(&self, ray: &Ray, shapes: &[Shape]) -> Option<Hit> {
        //return self.hit_fast(ray, ray.direction.recip(), shapes); // TODO SPEED Figure out if this is a worth while optimization. I think the compiller is doing it by itself

        if !self.aabb.hit(ray) {
            return None;
        }

        match &self.kind {
            BVHKind::Node(left, right) => match (left.hit(ray, shapes), right.hit(ray, shapes)) {
                (None, None) => None,
                (None, Some(h)) => Some(h),
                (Some(h), None) => Some(h),
                (Some(h1), Some(h2)) => {
                    if h1.t < h2.t {
                        Some(h1)
                    } else {
                        Some(h2)
                    }
                }
            },
            BVHKind::Leaf(shape_ref) => shapes[*shape_ref].hit(ray),
        }
    }

    fn hit_fast(&self, ray: &Ray, ray_dir_recip: Vector, shapes: &[Shape]) -> Option<Hit> {
        if !self.aabb.hit_fast(ray.origin, ray_dir_recip) {
            return None;
        }

        match &self.kind {
            BVHKind::Node(left, right) => match (
                left.hit_fast(ray, ray_dir_recip, shapes),
                right.hit_fast(ray, ray_dir_recip, shapes),
            ) {
                (None, None) => None,
                (None, Some(h)) => Some(h),
                (Some(h), None) => Some(h),
                (Some(h1), Some(h2)) => {
                    if h1.t < h2.t {
                        Some(h1)
                    } else {
                        Some(h2)
                    }
                }
            },
            BVHKind::Leaf(shape_ref) => shapes[*shape_ref].hit(ray),
        }
    }

    fn create_bvh(aabbs: &mut [(ShapeRef, AABB)]) -> Box<BVH> {
        let aabb = aabbs
            .into_iter()
            .map(|(_, aabb)| *aabb)
            .reduce(|a, b| a.surrounding_box(&b))
            .unwrap();

        let axis = nanorand::tls_rng().generate_range(0..3);

        let kind = if aabbs.len() == 1 {
            BVHKind::Leaf(aabbs.first().unwrap().0)
        } else {
            aabbs.sort_unstable_by(|(_, a), (_, b)| a.min[axis].partial_cmp(&b.min[axis]).unwrap());

            let half = aabbs.len() / 2;
            let left = Self::create_bvh(&mut aabbs[..half]);
            let right = Self::create_bvh(&mut aabbs[half..]);
            BVHKind::Node(left, right)
        };

        Box::new(BVH { aabb, kind })
    }
}
