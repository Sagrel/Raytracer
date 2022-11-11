use crate::ray::{Hit, Ray};
use crate::vec3::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Material {
    Dielectric(f64),
    Metal(Vec3, f64),
    Diffuse(Vec3), // Lambertian
}

fn schilck(cos: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

impl Material {
    pub fn scatter(self, ray: &Ray, hit: &Hit) -> Option<(Ray, Vec3)> {
        match self {
            Material::Dielectric(ref_idx) => {
                /*
                let refraction_ratio = if hit.front_face {
                    1.0 / ref_idx
                } else {
                    ref_idx
                };
                let cos_theta = f64::min(Vec3::dot(ray.direction * -1.0, hit.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refact = refraction_ratio * sin_theta > 1.0;

                let direction = if cannot_refact
                    || schilck(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f64>()
                {
                    Vec3::reflect(ray.direction, hit.normal)
                } else {
                    Vec3::refract(ray.direction, hit.normal, refraction_ratio)
                };
                Some((Ray::new(hit.point, direction), Vec3::new(1.0, 1.0, 1.0)))
                */

                let reflected = Vec3::reflect(ray.direction, hit.normal);
                let atenuation = Vec3::new(1.0, 1.0, 1.0);
                let normal: Vec3;
                let ni_over_nt: f64;
                let cos: f64;

                if Vec3::dot(ray.direction, hit.normal) > 0.0 {
                    normal = Vec3::zero() - hit.normal;
                    ni_over_nt = ref_idx;
                    cos = ref_idx * Vec3::dot(ray.direction, hit.normal);
                } else {
                    normal = hit.normal;
                    ni_over_nt = 1.0 / ref_idx;
                    cos = -Vec3::dot(ray.direction, hit.normal);
                }

                let refracted = Vec3::refract(ray.direction, normal, ni_over_nt);

                match Some(refracted) {
                    Some(r) => {
                        // SPEED This uses the thread local RNG
                        if fastrand::f64() < schilck(cos, ref_idx) {
                            Some((Ray::new(hit.point, r), atenuation))
                        } else {
                            Some((Ray::new(hit.point, reflected), atenuation))
                        }
                    }
                    None => Some((Ray::new(hit.point, reflected), atenuation)),
                }
            }
            Material::Metal(albedo, fuzz) => {
                let reflected = Vec3::reflect(ray.direction, hit.normal);
                let scatered =
                    Ray::new(hit.point, reflected + Vec3::random_in_unit_sphere() * fuzz);
                if Vec3::dot(scatered.direction, hit.normal) > 0.0 {
                    Some((scatered, albedo))
                } else {
                    None
                }
            }
            Material::Diffuse(albedo) => {
                let scatter_direction = hit.normal + Vec3::random_in_unit_sphere().normalized();
                if scatter_direction.near_zero() {
                    Some((Ray::new(hit.point, hit.normal), albedo))
                } else {
                    Some((Ray::new(hit.point, scatter_direction), albedo))
                }
            }
        }
    }
}
