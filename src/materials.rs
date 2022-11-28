use crate::hit::HitInfo;
use crate::ray::Ray;
use crate::Real;
use crate::Vector;
use nanorand::Rng;
use serde::{Deserialize, Serialize};

pub type MaterialRef = usize;

fn reflect(a: Vector, b: Vector) -> Vector {
    a - b * a.dot(b) * 2.0
}

fn refract(a: Vector, b: Vector, ni_over_nt: Real) -> Vector {
    let cos_theta = b.dot(-a).min(1.0);
    let perpendicular = (a + b * cos_theta) * ni_over_nt;
    let parallel = -(1.0 - perpendicular.length_squared()).abs().sqrt() * b;
    parallel + perpendicular
}

fn random_in_unit_sphere() -> Vector {
    // SPEED Is this the best way?
    let mut rng = nanorand::tls_rng();

    loop {
        let v = Vector::new(
            rng.generate::<Real>(),
            rng.generate::<Real>(),
            rng.generate::<Real>(),
        ) * 2.0
            - Vector::ONE;

        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

// TODO Create convenience constructor funcitions that take Into<Vector> so we can use tuples and stuff like that
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Material {
    Dielectric(Real),    // Cristal
    Metal(Vector, Real), // Metal/Mirror
    Diffuse(Vector),     // Lambertian, rough surface
}

fn reflectance(cos: Real, ref_idx: Real) -> Real {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

fn near_zero(v: Vector) -> bool {
    v.x.abs() < Real::EPSILON && v.y.abs() < Real::EPSILON && v.z.abs() < Real::EPSILON
}

impl Material {
    pub fn scatter(self, ray: &Ray, hit: &HitInfo) -> Option<(Ray, Vector)> {
        match self {
            Material::Dielectric(ref_idx) => {
                // TODO This seems to be broken again. At some point I got it working, let´s look at the git history
                let refraction_ratio = if hit.front_face {
                    ref_idx.recip()
                } else {
                    ref_idx
                };

                let cos_theta = Real::min(hit.normal.dot(-ray.direction), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refact = refraction_ratio * sin_theta > 1.0;

                let direction = if cannot_refact
                    || reflectance(cos_theta, refraction_ratio)
                        > nanorand::tls_rng().generate::<Real>()
                {
                    reflect(ray.direction, hit.normal)
                } else {
                    refract(ray.direction, hit.normal, refraction_ratio)
                };
                Some((Ray::new(hit.point, direction), Vector::ONE))
            }
            Material::Metal(albedo, fuzz) => {
                let reflected = reflect(ray.direction, hit.normal);
                let scatered = Ray::new(hit.point, reflected + random_in_unit_sphere() * fuzz);
                if scatered.direction.dot(hit.normal) > 0.0 {
                    Some((scatered, albedo))
                } else {
                    None
                }
            }
            Material::Diffuse(albedo) => {
                let scatter_direction = hit.normal + random_in_unit_sphere().normalize();
                let direction = if near_zero(scatter_direction) {
                    hit.normal
                } else {
                    scatter_direction
                };
                Some((Ray::new(hit.point, direction), albedo))
            }
        }
    }
}
