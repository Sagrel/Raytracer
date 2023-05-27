// TODO Cornell box

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use easy_gltf::model::Vertex;
    use nanorand::{tls::TlsWyRand, Rng};

    use crate::{
        materials::*,
        scene::Scene,
        shapes::{Shape, ShapeKind},
        Real, Vector,
    };

    impl Scene {
        pub fn new(look_from: Vector, look_at: Vector, fov: Real) -> Self {
            Self {
                shapes: Vec::new(),
                materials: Vec::new(),
                look_from,
                look_at,
                fov,
            }
        }
        pub fn add_material(&mut self, material: Material) -> MaterialRef {
            self.materials.push(material);

            self.materials.len() - 1
        }
    }

    pub fn dielectric(a: Real) -> Material {
        Material::Dielectric(a)
    }

    pub fn metal(v: impl Into<Vector>, a: Real) -> Material {
        Material::Metal(v.into(), a)
    }

    pub fn diffuse(v: impl Into<Vector>) -> Material {
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

    fn save_world(name: &str, scene: Scene) {
        File::create(format!("./scenes/{name}.json"))
            .unwrap()
            .write_all(serde_json::to_string_pretty(&scene).unwrap().as_bytes())
            .expect("Could not write file");
    }

    #[test]
    fn create_scene_one() {
        save_world("scene_one", scene_one())
    }

    #[test]
    fn create_scene_two() {
        save_world("scene_two", scene_two())
    }

    fn random_color(rng: &mut TlsWyRand) -> Vector {
        Vector::new(
            rng.generate::<Real>(),
            rng.generate::<Real>(),
            rng.generate::<Real>(),
        )
    }

    fn to_vector(vertex: Vertex) -> Vector {
        (
            vertex.position.x as Real,
            vertex.position.y as Real,
            vertex.position.z as Real,
        )
            .into()
    }

    fn load_model(name: &str) -> impl Iterator<Item = ShapeKind> {
        let scenes = easy_gltf::load(format!("./models/{name}/{name}.glb")).unwrap();
        scenes.into_iter().flat_map(|s| {
            s.models.into_iter().flat_map(|m| {
                m.triangles()
                    .unwrap()
                    .into_iter()
                    .map(|[a, b, c]| ShapeKind::Triangle(to_vector(a), to_vector(b), to_vector(c)))
            })
        })
    }

    fn scene_two() -> Scene {
        let mut scene = Scene::new(Vector::new(1.5, 3.0, 6.0), Vector::ZERO, 20.0);

        let material = scene.add_material(diffuse((0.5, 0.5, 0.5)));

        for triangle in load_model("Box") {
            scene.shapes.push(triangle.with_mat(material))
        }

        scene
    }

    fn scene_one() -> Scene {
        let mut scene = Scene::new(Vector::new(13.0, 2.0, 3.0), Vector::ZERO, 20.0);

        let ground_material = scene.add_material(diffuse((0.5, 0.5, 0.5)));
        scene
            .shapes
            .push(ShapeKind::Sphere((0.0, -1000.0, 0.0).into(), 1000.0).with_mat(ground_material));

        let mut rng = nanorand::tls_rng();

        for a in -11..11 {
            for b in -11..11 {
                let a = a as Real;
                let b = b as Real;
                let choose_mat = rng.generate::<Real>();
                let center = Vector::new(
                    a + 0.9 * rng.generate::<Real>(),
                    0.2,
                    b + 0.9 * rng.generate::<Real>(),
                );

                if (center - Vector::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let material = scene.add_material(if choose_mat < 0.8 {
                        // diffuse
                        let albedo = random_color(&mut rng) * random_color(&mut rng);
                        Material::Diffuse(albedo)
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = random_color(&mut rng) * 0.5 + Vector::new(0.5, 0.5, 0.5);
                        let fuzz = rng.generate::<Real>() * 0.5;
                        metal(albedo, fuzz)
                    } else {
                        // glass
                        dielectric(1.5)
                    });
                    let shape = if rng.generate::<bool>() {
                        ShapeKind::Triangle(
                            center + Vector::X,
                            center + Vector::Y,
                            center + Vector::Z,
                        )
                    } else {
                        ShapeKind::Sphere(center, 0.2)
                    };

                    scene.shapes.push(shape.with_mat(material));
                }
            }
        }
        let mat = scene.add_material(dielectric(1.5));
        scene
            .shapes
            .push(ShapeKind::Sphere(Vector::new(0.0, 1.0, 0.0), 1.0).with_mat(mat));

        let mat = scene.add_material(diffuse((0.4, 0.2, 0.1)));
        scene
            .shapes
            .push(ShapeKind::Sphere(Vector::new(-4.0, 1.0, 0.0), 1.0).with_mat(mat));

        let mat = scene.add_material(metal((0.7, 0.6, 0.5), 0.0));
        scene
            .shapes
            .push(ShapeKind::Sphere(Vector::new(4.0, 1.0, 0.0), 1.0).with_mat(mat));

        scene
    }
}
