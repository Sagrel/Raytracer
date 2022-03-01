use crate::vec3::Vec3;
use crate::shapes::Shape;
use crate::materials::Material;

#[derive(Debug, Copy, Clone)]
pub struct Ray
{
    pub origin : Vec3,
    pub direction : Vec3
}

#[derive(Debug, Copy, Clone)]
pub struct Hit
{
    pub t : f32,
    pub point : Vec3,
    pub normal : Vec3,
    pub material : Material
}

impl Hit
{
    pub fn new(t : f32, p : Vec3, n : Vec3, m : Material) -> Hit
    {
        Hit{t, point : p, normal : n, material : m}
    }
}

impl Ray
{
    pub fn new(o : Vec3, d : Vec3) -> Ray
    {
        Ray{origin : o, direction : d.normalized()}
    }

    pub fn point(&self, t : f32) -> Vec3
    {
        self.origin + self.direction * t
    }

    pub fn first_hit(&self, world : &[Shape]) -> Option<Hit>
    {
        let mut closest : Option<Hit> = None;

        for shape in world 
        {
            let hit = shape.intersects(self);

            match (hit, closest)
            {
                (Some(h), Some(c)) =>  if h.t < c.t { closest = hit },
                (Some(_), None) =>  closest = hit ,
                _ => ()
            }
        }

        closest
    }

    pub fn bounce(&self, world : &[Shape], ambient_light : Vec3, ttl : i32) -> Vec3
    {
        if ttl <= 0 { return ambient_light; }

        let hit = self.first_hit(world);

        match hit
        {
            Some(h) =>  
            {
                let res = h.material.scatter(self, h);

                match res
                {
                    Some((scattered, attenuation)) =>
                    {
                        attenuation * scattered.bounce(world, ambient_light, ttl - 1)
                    },
                    None =>  Vec3::zero()
                }
            }
            None =>
            {
                let t = 0.5 * (self.direction.y + 1.0);
                Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + ambient_light * t
            }   
        }          
    }
}