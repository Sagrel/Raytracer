use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::{materials::Material, shapes::Shape, Real, Vector};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub shapes: Vec<Shape>,
    pub materials: Vec<Material>,
    pub look_from: Vector,
    pub look_at: Vector,
    pub fov: Real,
}
// TODO A scene should ONLY contain the shapes and materials!

impl Scene {

    pub fn read_scene(scene: &str) -> Self {
        serde_json::from_reader(File::open(format!("./scenes/{scene}.json")).unwrap()).unwrap()
    }
}
