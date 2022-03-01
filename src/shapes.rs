use crate::vec3::Vec3;
use crate::ray::{Ray, Hit};
use crate::materials::Material;

pub enum Shape
{    
    Sphere(Vec3, f32, Material),
    Plane(Vec3, f32, Material)
}

impl Shape
{    
    pub fn intersects(&self, ray : &Ray) -> Option<Hit>
    {
        match self
        {
            Shape::Sphere(center, radious, mat) => 
            {
                let o_c = ray.origin - *center;
                let a = Vec3::dot(ray.direction, ray.direction);
                let b =  Vec3::dot(ray.direction, o_c) * 2.0;
                let c = Vec3::dot(o_c, o_c)  - radious * radious; 

                let mut raiz = b * b - 4.0 * a * c;

                if raiz.abs() < 0.05 { raiz = 0.0; }

                if raiz < 0.0 { return None; }

                raiz = raiz.sqrt();

                let mut t = (-b - raiz) / (2.0 * a);
                if t <= 0.0 {t = (-b + raiz) / (2.0 * a)};
                if t <= 0.0 { return None; }

                let point = ray.point(t);

                Some(Hit::new(t, point, (point - *center).normalized(), *mat)) 
            },
            Shape::Plane(normal, distance, mat) => 
            {
                let d = Vec3::dot(*normal, ray.direction);
                let pos = (*normal) * (*distance);

                if d.abs() < 0.05 { return None; }

                let t = Vec3::dot(pos - ray.origin, *normal) / d;

                if t < 0.0 { return None; }

                Some(Hit::new(t, ray.point(t), *normal, *mat))
            }
        }
    }
}
