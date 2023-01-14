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

mod particle;
mod gui;
mod camera;
mod renderer;
mod controller;

mod particle_settings;

use particle::Particle;
use particle_settings::{ParticleSettings, ParticleWrapping};
use gui::GUI;
use camera::Camera;
use renderer::{Renderer, MAX_INSTANCES, MAX_COLORS};
use controller::Controller;

const MAX_WIDTH: f32 = 2500.0;
const MAX_HEIGHT: f32 = 2500.0;

struct Game {
    renderer: Renderer,
    camera: Camera,
    controller: Controller,

    delta_time: f32,
    last_frame_time: std::time::SystemTime,

    particles: Vec<Particle>,
    particle_settings: ParticleSettings, 

    show_ui: bool
}

impl Game {
    async fn new(window: &Window) -> Self {
        let controller = Controller::new();

        let camera = Camera::new();
        
        let pallettes = vec![
            vec![
                glm::Vec3::new(1.0, 0.1, 0.1),
                glm::Vec3::new(0.1, 1.0, 0.1),
                glm::Vec3::new(0.1, 0.1, 1.0),
                glm::Vec3::new(0.5, 0.1, 1.0),
                glm::Vec3::new(1.0, 0.1, 0.5),
                glm::Vec3::new(1.0, 1.0, 0.1),
            ],
            vec![
                glm::Vec3::new(1.0, 0.0, 0.1),
                glm::Vec3::new(0.1, 0.0, 1.0),
            ],
            vec![
                glm::Vec3::new(1.0, 0.0, 0.1),
                glm::Vec3::new(1.0, 0.6, 0.1),
                glm::Vec3::new(1.0, 0.2, 0.9),
                glm::Vec3::new(0.1, 0.0, 1.0),
                glm::Vec3::new(0.1, 0.6, 1.0),
                glm::Vec3::new(0.5, 0.6, 1.0),
                glm::Vec3::new(0.1, 1.0, 0.0),
                glm::Vec3::new(0.1, 1.0, 0.6),
                glm::Vec3::new(0.85, 1.0, 0.6),
            ],
            vec![
                glm::Vec3::new(1.0, 0.0, 0.1),
                glm::Vec3::new(0.0, 0.6, 1.0),
                glm::Vec3::new(0.7, 1.0, 0.1),
                glm::Vec3::new(1.0, 0.2, 0.9),
            ],
        ];

        let colors = pallettes[1].clone();
        
        assert!(colors.len() < MAX_COLORS);

        let particles = Self::new_particles(colors.len());
        let color_table = Self::new_color_table(colors.len());

        let particle_settings = ParticleSettings { 
            colors,
            color_table,
            max_r: 250.0,
            min_r: 50.0,
            force: 5.0,
            drag: 0.0000001,
            radius: 20.0,
            wrapping: ParticleWrapping::Barrier,
        };

        let renderer = Renderer::new(window, &particle_settings.colors).await;

        println!("Game initialized!");
    
        Self {
            renderer,
            camera,
            controller,

            delta_time: 0.0,
            last_frame_time: std::time::SystemTime::now(),

            particles,
            particle_settings,

            show_ui: true
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, scale_factor: f64) {
        if new_size.width > 0 && new_size.height > 0 {
            println!("Window resized to (x: {}, y: {})", new_size.width, new_size.height);

            self.renderer.resize(new_size, scale_factor);

            self.camera.size.x = new_size.width as f32;
            self.camera.size.y = new_size.height as f32;
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        self.controller.process_input(event)
    }

    fn update(&mut self) {
        self.delta_time = self.last_frame_time.elapsed().unwrap().as_secs_f32();
        self.last_frame_time = std::time::SystemTime::now();

        self.camera.zoom -= self.controller.mouse_wheel * 0.025 * self.camera.zoom;

        self.camera.zoom = self.camera.zoom.clamp(0.1, 20.0);

        self.controller.mouse_wheel = 0.0;

        let camera_speed = 400.0 * self.delta_time * self.camera.zoom;

        if self.controller.is_key_pressed(VirtualKeyCode::W) {
            self.camera.position.y += camera_speed;
        } else if self.controller.is_key_pressed(VirtualKeyCode::S) {
            self.camera.position.y -= camera_speed;
        }

        if self.controller.is_key_pressed(VirtualKeyCode::D) {
            self.camera.position.x += camera_speed;
        } else if self.controller.is_key_pressed(VirtualKeyCode::A) {
            self.camera.position.x -= camera_speed;
        }

        if self.controller.is_key_pressed(VirtualKeyCode::R) {
            self.particles = Self::new_particles(self.particle_settings.colors.len());
        }
        if self.controller.is_key_pressed(VirtualKeyCode::T) {
            self.particle_settings.color_table = Self::new_color_table(self.particle_settings.colors.len());
        }

        let other_particles = self.particles.clone();

        let start = Instant::now();

        self.particles.par_iter_mut().for_each(|p| {
            p.velocity *= self.particle_settings.drag.powf(self.delta_time);

            for other in &other_particles {
                let diff: glm::Vec2 = other.position - p.position;
                let dist: f32 = (diff.x*diff.x+diff.y*diff.y).sqrt();

                if dist < 0.0001 || dist > self.particle_settings.max_r {
                    continue;
                } 

                let dir: glm::Vec2 = glm::Vec2::new(diff.x / dist, diff.y / dist);

                // https://www.desmos.com/calculator/yacrclthei?lang=pl
                if dist > self.particle_settings.min_r {
                    let c = self.particle_settings.color_table[p.color_id as usize][other.color_id as usize];
                    p.velocity += self.particle_settings.force * dir * c * ((PI*(dist - self.particle_settings.min_r)) / (self.particle_settings.max_r - self.particle_settings.min_r)).sin();
                } else {
                    p.velocity += self.particle_settings.force * dir * (dist / self.particle_settings.min_r - 1.0);
                }
            }
        });

        let velocity_update = start.elapsed().as_secs_f64()*1000.0;

        let start = Instant::now();

        for particle in &mut self.particles {
            particle.position += particle.velocity * self.delta_time;

            match self.particle_settings.wrapping {
                ParticleWrapping::None => {}
                ParticleWrapping::Wrap => {
                    if particle.position.x.abs() > MAX_WIDTH {
                        particle.position.x = -particle.position.x;
                    }

                    if particle.position.y.abs() > MAX_HEIGHT {
                        particle.position.y = -particle.position.y;
                    }
                }
                ParticleWrapping::Barrier => {
                    particle.position.x = particle.position.x.clamp(-MAX_WIDTH, MAX_WIDTH);
                    particle.position.y = particle.position.y.clamp(-MAX_HEIGHT, MAX_HEIGHT);
                }
            }
        }

        let position_update = start.elapsed().as_secs_f64()*1000.0;

        print!("Physics update took: {:.2}ms (velocity update: {:.2}ms, position update: {:.2}ms) ", velocity_update + position_update, velocity_update, position_update);
    }

    fn render(&mut self, gui: &mut GUI) -> Result<(), wgpu::SurfaceError> {
        for particle in &self.particles {
            self.renderer.enqueue_instance(renderer::Instance {
                position: particle.position,
                radius: self.particle_settings.radius,
                color_id: particle.color_id as u32
            });
        }

        let frame_data = if self.show_ui {
            gui.draw_ui(&mut self.particle_settings)
        } else {
            None
        };

        let result = self.renderer.render(self.camera.calc_matrices(), frame_data);

        result
    }

    fn new_color_table(color_count: usize) -> Vec<Vec<f32>> {
        (0..color_count).map(|_| {
            (0..color_count).map(|_| {
                rand::thread_rng().gen_range(-1.0..=1.0)
            }).collect()
        }).collect()
    }

    fn new_particles(color_count: usize) -> Vec<Particle> {
        (0..MAX_INSTANCES).map(|_| {
            Particle::new(
                glm::Vec2::new(
                    rand::thread_rng().gen_range(-MAX_WIDTH ..=MAX_WIDTH ) / 2.0 + rand::thread_rng().gen_range(-MAX_WIDTH ..=MAX_WIDTH ) / 2.0,
                    rand::thread_rng().gen_range(-MAX_HEIGHT..=MAX_HEIGHT) / 2.0 + rand::thread_rng().gen_range(-MAX_HEIGHT..=MAX_HEIGHT) / 2.0
                ), 
                rand::thread_rng().gen_range(0..color_count as u8)
            )
        }).collect()
    }
}

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_inner_size(LogicalSize::new(1200, 1000));
    window.set_title("Particle Life");

    let mut gui = GUI::new(&window);

    let mut game = Game::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        gui.handle_event(&event);

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                game.update();

                match game.render(&mut gui) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => game.resize(
                        winit::dpi::PhysicalSize::new(
                            game.camera.size.x as u32,
                            game.camera.size.y as u32
                        ),
                        window.scale_factor()
                    ),
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
                        game.resize(*physical_size, window.scale_factor());
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(run());
}