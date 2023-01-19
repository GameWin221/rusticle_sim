use serde::{Serialize, Deserialize};
use rand::{distributions::Alphanumeric, Rng};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParticleWrapping {
    Barrier,
    Wrap,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldSettings {
    pub max_particles: usize,
    pub size: f32,
    pub wrapping: ParticleWrapping,
    pub seed: String,
}

impl WorldSettings {
    fn random_seed() -> String {
        rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect()
    }

    pub fn new_random_seed(&mut self) {
        self.seed = Self::random_seed();
    }
}

impl Default for WorldSettings {
    fn default() -> Self {
        Self { 
            max_particles: 10000,
            size: 5000.0,
            wrapping: ParticleWrapping::Wrap,
            seed: Self::random_seed(),
        }
    }
}