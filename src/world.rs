use serde::{Deserialize, Serialize};

use crate::{materials::Material, shapes::Shape};

#[derive(Default, Serialize, Deserialize)]
pub struct World {
    pub shapes: Vec<Shape>,
    pub materials: Vec<Material>,
}
