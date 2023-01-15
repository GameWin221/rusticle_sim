extern crate nalgebra_glm as glm;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub enum ParticleWrapping {
    #[default]
    Barrier,
    Wrap,
}

#[derive(Clone, Default, Debug)]
pub struct ParticleSettings {
    pub colors: Vec<glm::Vec3>,
    pub color_table: Vec<Vec<f32>>,

    pub max_r: f32,
    pub min_r: f32,
    pub force: f32,
    pub drag: f32,

    pub radius: f32,

    pub wrapping: ParticleWrapping,
}