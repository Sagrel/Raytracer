use serde::{Deserialize, Serialize};

use crate::{camera::Camera, vec3::Vec3};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub camera: Camera,
    pub scene: String,
    pub ambient_color: Vec3,
    pub width: usize,
    pub height: usize,
    pub samples: usize,
    pub ttl: usize,
    pub chunk_size: usize,
    pub bvh_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        let camera = Camera::new(
            (-2.0, 2.0, 1.0),
            (0.0, 0.0, -1.0),
            (0.0, 1.0, 0.0),
            90.0,
            RATIO,
        );
        let ambient_color = (0.5, 0.7, 1.0).into();
        const WIDHT: usize = 20; //1366;
        const HEIGHT: usize = 10; //768;
        const SAMPLES: usize = 10;
        const CHUNK_ROWS: usize = 2;
        const RATIO: f64 = WIDHT as f64 / HEIGHT as f64;
        const TTL: usize = 1024;

        Self {
            camera,
            ambient_color,
            scene: "scene_one".to_string(),
            width: WIDHT,
            height: HEIGHT,
            samples: SAMPLES,
            chunk_size: CHUNK_ROWS,
            ttl: TTL,
            bvh_enabled: true,
        }
    }
}
