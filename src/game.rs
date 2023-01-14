extern crate nalgebra_glm as glm;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    window::Window,
    dpi::LogicalSize
};

use std::{time::Instant, f32::consts::PI};

use rayon::prelude::*;

use rand::Rng;

#[path="particle.rs"]   mod particle;
#[path="camera.rs"]     mod camera;
#[path="renderer.rs"]   mod renderer;
#[path="controller.rs"] mod controller;

use particle::Particle;
use camera::Camera;
use renderer::{Renderer, MAX_INSTANCES};
use controller::Controller;

use crate::game::renderer::MAX_COLORS;

const MAX_WIDTH: f32 = 2500.0;
const MAX_HEIGHT: f32 = 2500.0;

struct Game {
    renderer: Renderer,
    camera: Camera,
    controller: Controller,

    delta_time: f32,
    last_frame_time: std::time::SystemTime,

    particles: Vec<Particle>,

    particle_colors: Vec<glm::Vec3>,
    color_table: Vec<Vec<f32>>,

    particle_max_r: f32,
    particle_min_r: f32,

    particle_friction: f32
}

impl Game {
    async fn new(window: &Window) -> Self {

        let controller = controller::Controller::new();

        let mut camera = camera::Camera::new(window.inner_size().width as f32, window.inner_size().height as f32);

        camera.position = glm::Vec2::new(0.0, 0.0);

        let particle_colors = vec![
            glm::Vec3::new(1.0, 0.1, 0.1),
            glm::Vec3::new(0.1, 1.0, 0.1),
            glm::Vec3::new(0.1, 0.1, 1.0),
            glm::Vec3::new(0.5, 0.1, 1.0),
            glm::Vec3::new(1.0, 0.1, 0.5),
            glm::Vec3::new(1.0, 1.0, 0.1),
        ];

        assert!(particle_colors.len() < MAX_COLORS);

        // Y - this particle, X - other particle
        let mut color_table = vec![vec![-0.25; particle_colors.len()]; particle_colors.len()];

        let color_count = particle_colors.len();

        // Y - this particle, X - other particle
        for y in 0..color_count {
            for x in 0..color_count {
                let v: f32 = rand::thread_rng().gen_range(-1.0..=1.0) * 5.0;
                color_table[y][x] = v.clamp(-1.0, 1.0);
            }
        }

        let mut particles = Vec::with_capacity(MAX_INSTANCES);

        for _ in 0..MAX_INSTANCES {
            let position = glm::Vec2::new(
                rand::thread_rng().gen_range(-2000.0..2000.0),
                rand::thread_rng().gen_range(-2000.0..2000.0)
            );

            let color_id = rand::thread_rng().gen_range(0..particle_colors.len() as u8);

            particles.push(Particle::new(position, color_id));
        }

        let renderer = renderer::Renderer::new(window, &particle_colors).await;

        println!("Game initialized!");
    
        Self {
            renderer,
            camera,
            controller,

            delta_time: 0.0,
            last_frame_time: std::time::SystemTime::now(),

            particles,
            particle_colors,
            color_table,

            particle_max_r: 250.0,
            particle_min_r: 50.0,

            particle_friction: 0.0000001
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            println!("Resizing to (x: {}, y: {})", new_size.width, new_size.height);

            self.renderer.resize(new_size);

            self.camera.size.x = new_size.width as f32;
            self.camera.size.y = new_size.height as f32;
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        self.controller.process_input(event)
    }

    fn reset_color_table(&mut self) {
        let color_count = self.particle_colors.len();

        // Y - this particle, X - other particle
        for y in 0..color_count {
            for x in 0..color_count {
                let v: f32 = rand::thread_rng().gen_range(-1.0..=1.0) * 5.0;
                self.color_table[y][x] = v.clamp(-1.0, 1.0);
            }
        }
    }

    fn reset_particles(&mut self) {
        let mut particles = Vec::with_capacity(MAX_INSTANCES);

        for _ in 0..MAX_INSTANCES {
            let position = glm::Vec2::new(
                rand::thread_rng().gen_range(-2000.0..2000.0),
                rand::thread_rng().gen_range(-2000.0..2000.0)
            );

            let color_id = rand::thread_rng().gen_range(0..self.particle_colors.len() as u8);

            particles.push(Particle::new(position, color_id));
        }

        self.particles = particles;
    }

    fn update(&mut self) {
        self.delta_time = self.last_frame_time.elapsed().unwrap().as_secs_f32();
        self.last_frame_time = std::time::SystemTime::now();

        self.camera.zoom -= self.controller.mouse_wheel * 20.0 * self.delta_time;

        self.camera.zoom = self.camera.zoom.clamp(0.1, 20.0);

        self.controller.mouse_wheel = 0.0;

        if self.controller.is_up_pressed {
            self.camera.position.y += 200.0 * self.delta_time * self.camera.zoom;
        } else if self.controller.is_down_pressed {
            self.camera.position.y -= 200.0 * self.delta_time * self.camera.zoom;
        }

        if self.controller.is_right_pressed {
            self.camera.position.x += 200.0 * self.delta_time * self.camera.zoom;
        } else if self.controller.is_left_pressed {
            self.camera.position.x -= 200.0 * self.delta_time * self.camera.zoom;
        }

        if self.controller.is_r_pressed {
            self.reset_particles();
        }
        if self.controller.is_t_pressed {
            self.reset_color_table();
        }

        if self.controller.is_space_pressed && self.particles.len() < MAX_INSTANCES {
            let m_pos = glm::Vec2::new(
                (self.controller.mouse_position.0 as f32 / self.camera.size.x) * 2.0 - 1.0,
                (1.0 - self.controller.mouse_position.1 as f32 / self.camera.size.y) * 2.0 - 1.0
            );

            let w_pos = self.camera.viewport_to_world(m_pos);

            let color_id = rand::thread_rng().gen_range(0..self.particle_colors.len() as u8);

            self.particles.push(Particle::new(w_pos, color_id));
        }

        let start = Instant::now();

        let other_particles = self.particles.clone();

        self.particles.par_iter_mut().for_each(|p| {
            p.velocity *= self.particle_friction.powf(self.delta_time);

            for other in &other_particles {
                let diff: glm::Vec2 = other.position - p.position;
                let dist: f32 = (diff.x*diff.x+diff.y*diff.y).sqrt();//diff.magnitude();

                if dist < 0.0001 || dist > self.particle_max_r {
                    continue;
                } 

                let dir: glm::Vec2 = glm::Vec2::new(diff.x / dist, diff.y / dist);

                // https://www.desmos.com/calculator/yacrclthei?lang=pl
                if dist > self.particle_min_r {
                    let c = self.color_table[p.color_id as usize][other.color_id as usize];
                    p.velocity += 5.0 * dir * c * ((PI*(dist - self.particle_min_r)) / (self.particle_max_r - self.particle_min_r)).sin();
                } else {
                    p.velocity += 10.0 * dir * (dist / self.particle_min_r - 1.0);
                }
            }
        });

/*
        for particle in &mut self.particles {
            for other in &other_particles {
                let distance = particle.position.metric_distance(&other.position);
                if distance < 0.000001 || distance > self.particle_max_r {
                    continue;
                } 

                let diff: glm::Vec2 = other.position - particle.position;
                let dir:  glm::Vec2 = diff.normalize();
                let dist: f32 = diff.magnitude();

                // https://www.desmos.com/calculator/yacrclthei?lang=pl
                if distance > self.particle_min_r {
                    let c = self.color_table[particle.color_id as usize][other.color_id as usize];
                    particle.velocity += 5.0 * dir * c * ((PI*(dist - self.particle_min_r)) / (self.particle_max_r - self.particle_min_r)).sin();//2.0 * dir * dist * self.delta_time * self.color_table[particle.color_id as usize][other.color_id as usize];
                } else {
                    particle.velocity += 10.0 * dir * (dist / self.particle_min_r - 1.0);//2.0 * dir * dist * self.delta_time * self.color_table[particle.color_id as usize][other.color_id as usize];
                }
            }
        }
*/
        for particle in &mut self.particles {
            particle.position += particle.velocity * self.delta_time;

            particle.position.x = particle.position.x.clamp(-MAX_WIDTH, MAX_WIDTH) ;
            particle.position.y = particle.position.y.clamp(-MAX_HEIGHT, MAX_HEIGHT) ;
        }

        let duration = start.elapsed();

        println!("update took: {:?}", duration);
    }
    
    fn prepare_instances(&mut self) {
        for particle in &self.particles {
            self.renderer.enqueue_instance(renderer::Instance {
                position: particle.position,
                radius: 20.0,
                color_id: particle.color_id as u32
            });
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.prepare_instances();

        let result = self.renderer.render(self.camera.calc_matrices());

        self.renderer.reset_queue();

        result
    }
}

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_inner_size(LogicalSize::new(1200, 1000));
    window.set_title("Particle Life");

    let mut game = Game::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                game.update();

                match game.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => game.resize(winit::dpi::PhysicalSize{width: game.camera.size.x as u32, height: game.camera.size.y as u32}),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            
            Event::MainEventsCleared => window.request_redraw(),

            Event::WindowEvent {
                ref event,
                window_id
            } 

            if window_id == window.id() => if !game.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    WindowEvent::Resized(physical_size) => {
                        game.resize(*physical_size);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}