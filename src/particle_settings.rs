use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticleSettings {
    pub max_r: f32,
    pub min_r: f32,
    pub force: f32,
    pub drag: f32,

    pub radius: f32,
    pub sharpness: f32,
}

impl Default for ParticleSettings {
    fn default() -> Self {
        Self { 
            max_r: 250.0,
            min_r: 50.0,
            force: 5.0,
            drag: 0.06813,
            radius: 20.0,
            sharpness: 0.8,
        }
    }
}