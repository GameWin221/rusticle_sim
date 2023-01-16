extern crate nalgebra_glm as glm;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub enum ParticleWrapping {
    #[default]
    Barrier,
    Wrap,
}

#[derive(Clone, Debug)]
pub struct ParticleSettings {
    pub colors: Vec<glm::Vec3>,
    pub color_table: Vec<Vec<f32>>,

    pub max_particles: usize,

    pub max_r: f32,
    pub min_r: f32,
    pub force: f32,
    pub drag: f32,

    pub radius: f32,
    pub sharpness: f32,

    pub wrapping: ParticleWrapping,
}

impl Default for ParticleSettings {
    fn default() -> Self {
        Self { 
            colors: Vec::new(),
            color_table: Vec::new(),
            max_particles: 10000,
            max_r: 250.0,
            min_r: 50.0,
            force: 5.0,
            drag: 0.06813,
            radius: 20.0,
            sharpness: 0.8,
            wrapping: ParticleWrapping::Barrier,
        }
    }
}