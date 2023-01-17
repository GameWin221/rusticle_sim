#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub enum ParticleWrapping {
    #[default]
    Barrier,
    Wrap,
}

#[derive(Clone, Debug)]
pub struct ParticleSettings {
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