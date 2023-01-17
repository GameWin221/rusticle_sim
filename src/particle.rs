#[derive(Clone)]
pub struct Particle{
    pub position: glm::Vec2,
    pub velocity: glm::Vec2,
    pub color_id: u8
}

impl Particle {
    pub fn new(position: glm::Vec2, color_id: u8) -> Self {
        Self { 
            position,
            velocity: glm::Vec2::new(0.0, 0.0), 
            color_id,
        }
    }
}