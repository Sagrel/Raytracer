// Bounding volume hierarchy

use crate::{aabb::AABB, hit::Hit, ray::Ray, shapes::Shape};

type ShapeRef = usize;

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

    /*
    TODO

    The idea is to generate the list of aabbs and then in another function we can recursively create bounding voulumes that get smaller, by dividing the shape slice in some arbitrary axis. We only generate the AABB once at the beggining and use indeces to it. In the BVH we should use and enum with the variant of node and leaf, where the node as 2 child BVH and the leaf has one of many indeces to the array of shapes. This way we can check the BVH to reduce the number of shapes to check, basically doing bynary search. The construction of the BVH is expensive, but is is very fast to iterate. We could even use a Vector instead of pointers, but I don't know if that would be faster.
    */

    fn create_bvh(aabbs: &mut [(ShapeRef, AABB)]) -> Box<BVH> {
        let aabb = aabbs
            .into_iter()
            .map(|(_, aabb)| *aabb)
            .reduce(|a, b| a.surrounding_box(&b))
            .unwrap();

        let axis = fastrand::usize(0..3);

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
