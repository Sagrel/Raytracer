use crate::vec3::Vec3;
use crate::ray::{Ray, Hit};
use rand::Rng;


#[derive(Debug, Copy, Clone)]
pub enum Material
{
    Dielectric(f32),
    Metal(Vec3, f32),
    Diffuse(Vec3)
}

fn schilck(cos : f32, ref_idx : f32) -> f32
{
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}

impl Material
{
    pub fn scatter(self, ray : &Ray, hit : Hit) -> Option<(Ray, Vec3)>
    {
        match self
        {
            Material::Dielectric(ref_idx) => 
            {
                let reflected = Vec3::reflect(ray.direction, hit.normal);

                let normal : Vec3 ;
                let ni_over_nt : f32;
                let cos : f32;

                if Vec3::dot(ray.direction, hit.normal) > 0.0
                {
                    normal = Vec3::zero() - hit.normal;
                    ni_over_nt = ref_idx;
                    cos = ref_idx * Vec3::dot(ray.direction, hit.normal);
                }
                else
                {
                    normal = hit.normal;
                    ni_over_nt = 1.0 / ref_idx;
                    cos = -Vec3::dot(ray.direction, hit.normal);
                }

                let refracted = Vec3::refract(ray.direction, normal, ni_over_nt);
                
                match refracted
                {
                    Some(r) => 
                    {
                        let mut rng = rand::thread_rng();
                        if rng.gen::<f32>() < schilck(cos, ref_idx)
                        {
                            Some((Ray::new(hit.point, r), Vec3::new(1.0, 1.0, 1.0)))
                        } 
                        else
                        {
                            Some((Ray::new(hit.point, reflected), Vec3::new(1.0, 1.0, 1.0)))
                        }  
                    }
                    None => Some((Ray::new(hit.point, reflected), Vec3::new(1.0, 1.0, 1.0)))
                }
               
            },
            Material::Metal(albedo, fuzz) =>
            {
                let reflected = Vec3::reflect(ray.direction, hit.normal);

                if Vec3::dot(reflected, hit.normal) > 0.0
                {
                    Some((Ray::new(hit.point, reflected + Vec3::random_in_unit_sphere() * fuzz), albedo))
                }
                else { None }
            },
            Material::Diffuse(albedo) =>
            {
                let target = hit.point + hit.normal + Vec3::random_in_unit_sphere();
                
                Some((Ray::new(hit.point, target - hit.point), albedo))
            }

        }
    }
}
