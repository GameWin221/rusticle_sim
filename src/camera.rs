extern crate nalgebra_glm as glm;

use std::ops::RangeInclusive;

pub struct Camera {
    pub position: glm::Vec2,
    pub size: glm::Vec2,
    pub scale: f32,
    scale_limits: RangeInclusive<f32>
}

impl Camera {
    pub fn new(scale_limits: RangeInclusive<f32>) -> Self {
        Self {
            position: glm::Vec2::zeros(),
            size: glm::Vec2::zeros(),
            scale: 1.0,
            scale_limits
        }
    }

    pub fn zoom(&mut self, delta: f32) {
        self.scale -= delta * self.scale;
        self.scale = self.scale.clamp((*self.scale_limits.start()).into(), (*self.scale_limits.end()).into());
    }

    pub fn move_xy(&mut self, delta: glm::Vec2) {
        self.position += delta * self.scale;
    }

    pub fn viewport_to_world(&self, ndc: glm::Vec2) -> glm::Vec2 {
        self.position + (self.size.component_mul(&ndc) / 2.0 * self.scale)
    }

    pub fn calc_matrices(&self) -> glm::Mat4x4 {
        let view = glm::Mat4x4::look_at_rh(&glm::Vec3::new(self.position.x, self.position.y, 1.0).into(), &glm::Vec3::new(self.position.x, self.position.y, 0.0).into(), &glm::Vec3::new(0.0, 1.0, 0.0).into());
        
        let size = self.size * self.scale;

        let proj = glm::ortho_rh(
            -size.x/2.0, size.x/2.0,
            -size.y/2.0, size.y/2.0,
            0.1, 100.0
        );

        let gl_to_wgpu = glm::make_mat4x4(
            &[1.0, 0.0, 0.0, 0.0,
             0.0, 1.0, 0.0, 0.0,
             0.0, 0.0, 0.5, 0.0,
             0.0, 0.0, 0.5, 1.0]
        );
        
        gl_to_wgpu * proj * view
    }
}