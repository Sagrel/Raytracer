use serde::{Deserialize, Serialize};

use crate::{Real, Vector};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub scene: String,
    pub ambient_color: Vector,
    pub width: usize,
    pub height: usize,
    pub aspect_ratio: Real,
    pub samples: usize,
    pub ttl: usize,
    pub chunk_size: usize,
    pub bvh_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        let ambient_color = (0.5, 0.7, 1.0).into();
        const WIDHT: usize = 800;
        const HEIGHT: usize = 600;
        const SAMPLES: usize = 10;
        const CHUNK_ROWS: usize = 2;
        const RATIO: Real = WIDHT as Real / HEIGHT as Real;
        const TTL: usize = 64;

        Self {
            ambient_color,
            scene: "scene_one".to_string(),
            width: WIDHT,
            height: HEIGHT,
            samples: SAMPLES,
            chunk_size: CHUNK_ROWS,
            ttl: TTL,
            aspect_ratio: RATIO,
            bvh_enabled: true,
        }
    }
}
