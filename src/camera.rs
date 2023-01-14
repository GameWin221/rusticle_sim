extern crate nalgebra_glm as glm;

pub struct Camera {
    pub position: glm::Vec2,
    pub size: glm::Vec2,
    pub zoom: f32
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: glm::Vec2::zeros(),
            size: glm::Vec2::zeros(),
            zoom: 1.0
        }
    }

    pub fn viewport_to_world(&self, ndc: glm::Vec2) -> glm::Vec2 {
        self.position + (self.size.component_mul(&ndc) / 2.0 * self.zoom)
    }

    pub fn calc_matrices(&self) -> glm::Mat4x4 {
        let view = glm::Mat4x4::look_at_rh(&glm::Vec3::new(self.position.x, self.position.y, 1.0).into(), &glm::Vec3::new(self.position.x, self.position.y, 0.0).into(), &glm::Vec3::new(0.0, 1.0, 0.0).into());
        
        let size = self.size * self.zoom;

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