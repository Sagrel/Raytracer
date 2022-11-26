use crate::hit::Hit;
use crate::vec3::Vec3;
use crate::world::World;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
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

    pub fn bounce(
        &self,
        get_hit: &impl Fn(&Self) -> Option<Hit>,
        world: &World,
        ambient_color: &Vec3,
        ttl: usize,
    ) -> Vec3 {
        if ttl <= 0 {
            return *ambient_color;
        }

        match get_hit(self) {
            Some(h) => {
                let res = world.materials[h.material].scatter(self, &h);

                match res {
                    Some((scattered, attenuation)) => {
                        attenuation * scattered.bounce(get_hit, world, ambient_color, ttl - 1)
                    }
                    None => Vec3::zero(),
                }
            }
            None => {
                let t = 0.5 * (self.direction.y + 1.0);
                Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + *ambient_color * t
            }
        }
    }
    /*
    pub fn bounce(&self, world: &World, config: &Config, ttl: usize) -> Vec3 {
        if ttl <= 0 {
            return config.ambient_color;
        }

        let hit = if config.bvh_enabled { world.}

        match self.first_hit(&world.shapes) {
            Some(h) => {
                let res = world.materials[h.material].scatter(self, &h);

                match res {
                    Some((scattered, attenuation)) => {
                        attenuation * scattered.bounce(world, config, ttl - 1)
                    }
                    None => Vec3::zero(),
                }
            }
            None => {
                let t = 0.5 * (self.direction.y + 1.0);
                Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + config.ambient_color * t
            }
        }
    }
    */
}
