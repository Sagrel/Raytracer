use crate::materials::Material;
use crate::shapes::Shape;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Debug, Copy, Clone)]
pub struct Hit {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub front_face: bool,
}

impl Hit {
    pub fn new(t: f64, point: Vec3, normal: Vec3, material: Material, front_face: bool) -> Hit {
        Hit {
            t,
            point,
            normal,
            material,
            front_face
        }
    }
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.normalized(),
        }
    }

    pub fn point(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn first_hit(&self, world: &[Shape]) -> Option<Hit> {
        world
            .iter()
            .filter_map(|shape| shape.intersects(self))
            .reduce(|a, b| if a.t < b.t { a } else { b })
    }

    pub fn bounce(&self, world: &[Shape], ambient_light: &Vec3, ttl: i32) -> Vec3 {
        if ttl <= 0 {
            return *ambient_light;
        }

        match self.first_hit(world) {
            Some(h) => {
                let res = h.material.scatter(self, &h);

                match res {
                    Some((scattered, attenuation)) => {
                        attenuation * scattered.bounce(world, ambient_light, ttl - 1)
                    }
                    None => Vec3::zero(),
                }
            }
            None => {
                let t = 0.5 * (self.direction.y + 1.0);
                Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + *ambient_light * t
            }
        }
    }
}
