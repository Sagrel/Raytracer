use crate::bvh::BVH;
use crate::scene::Scene;
use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn point(&self, t: Real) -> Vector {
        self.origin + self.direction * t
    }

    pub fn bounce(&self, bvh: &BVH, scene: &Scene, ambient_color: &Vector, ttl: usize) -> Vector {
        if ttl <= 0 {
            return *ambient_color;
        }

        match bvh.hit(self, &scene.shapes) {
            Some(h) => {
                let res = scene.materials[h.material].scatter(self, &h.get_hit_info(self));

                match res {
                    Some((scattered, attenuation)) => {
                        attenuation * scattered.bounce(bvh, scene, ambient_color, ttl - 1)
                    }
                    None => Vector::ZERO,
                }
            }
            None => {
                let t = 0.5 * (self.direction.y + 1.0);
                Vector::splat(1.0) * (1.0 - t) + *ambient_color * t
            }
        }
    }
}
