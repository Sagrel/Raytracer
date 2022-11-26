// TODO Cornell box

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use fastrand::Rng;

    use crate::{
        materials::*,
        shapes::{Shape, ShapeKind},
        vec3::Vec3,
        world::World,
    };

    impl World {
        pub fn add_material(&mut self, material: Material) -> MaterialRef {
            self.materials.push(material);

            self.materials.len() - 1
        }
    }

    pub fn dielectric(a: f64) -> Material {
        Material::Dielectric(a)
    }

    pub fn metal(v: impl Into<Vec3>, a: f64) -> Material {
        Material::Metal(v.into(), a)
    }

    pub fn diffuse(v: impl Into<Vec3>) -> Material {
        Material::Diffuse(v.into())
    }
    impl ShapeKind {
        pub fn with_mat(self, material: MaterialRef) -> Shape {
            Shape {
                kind: self,
                material,
            }
        }
    }

    fn save_world(name: &str, world: World) {
        File::create(format!("./scenes/{name}.json"))
            .unwrap()
            .write_all(serde_json::to_string_pretty(&world).unwrap().as_bytes())
            .expect("Could not write file");
    }

    #[test]
    fn create_scene_one() {
        save_world("scene_one", scene_one())
    }

    fn scene_one() -> World {
        let mut world = World::default();

        let ground_material = world.add_material(diffuse((0.5, 0.5, 0.5)));
        world
            .shapes
            .push(ShapeKind::Sphere((0.0, -1000.0, 0.0).into(), 1000.0).with_mat(ground_material));

        let rng = Rng::new();
        for a in -11..11 {
            for b in -11..11 {
                let a = a as f64;
                let b = b as f64;
                let choose_mat = rng.f64();
                let center = Vec3::new(a + 0.9 * rng.f64(), 0.2, b + 0.9 * rng.f64());

                let random_color = || Vec3::new(rng.f64(), rng.f64(), rng.f64());

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let material = world.add_material(if choose_mat < 0.8 {
                        // diffuse
                        let albedo = random_color() * random_color();
                        Material::Diffuse(albedo)
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = random_color() * 0.5 + Vec3::new(0.5, 0.5, 0.5);
                        let fuzz = rng.f64() * 0.5;
                        metal(albedo, fuzz)
                    } else {
                        // glass
                        dielectric(1.5)
                    });
                    world
                        .shapes
                        .push(ShapeKind::Sphere(center, 0.2).with_mat(material));
                }
            }
        }
        let mat = world.add_material(dielectric(1.5));
        world
            .shapes
            .push(ShapeKind::Sphere(Vec3::new(0.0, 1.0, 0.0), 1.0).with_mat(mat));

        let mat = world.add_material(diffuse((0.4, 0.2, 0.1)));
        world
            .shapes
            .push(ShapeKind::Sphere(Vec3::new(-4.0, 1.0, 0.0), 1.0).with_mat(mat));

        let mat = world.add_material(metal((0.7, 0.6, 0.5), 0.0));
        world
            .shapes
            .push(ShapeKind::Sphere(Vec3::new(4.0, 1.0, 0.0), 1.0).with_mat(mat));

        world
    }
}
