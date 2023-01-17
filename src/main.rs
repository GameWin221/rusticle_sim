extern crate nalgebra_glm as glm;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    window::Window,
    dpi::LogicalSize
};

mod particle;
mod gui;
mod camera;
mod renderer;
mod controller;
mod world;

mod particle_settings;
mod color_table;

use world::World;
use particle_settings::ParticleSettings;
use color_table::ColorTable;
use gui::GUI;
use camera::Camera;
use renderer::{Renderer, MAX_INSTANCES, MAX_COLORS};
use controller::{Controller, Key, Button};

struct Game {
    renderer: Renderer,
    camera: Camera,
    controller: Controller,

    delta_time: f32,
    last_frame_time: std::time::SystemTime,

    //particles: Vec<Particle>,
    world: World,
    particle_settings: ParticleSettings, 
    color_table: ColorTable, 

    followed_index: Option<usize>,

    show_ui: bool
}

impl Game {
    async fn new(window: &Window) -> Self {
        let controller = Controller::new();

        let camera = Camera::new(1.0..=20.0);
        
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
            ],
        ];

        let color_table = ColorTable::new(pallettes[2].clone());

        let particle_settings = ParticleSettings::default();

        assert!(color_table.colors.len() < MAX_COLORS);
        assert!(particle_settings.max_particles < MAX_INSTANCES);

        let world = World::new(2500.0, particle_settings.max_r, particle_settings.max_particles);

        let renderer = Renderer::new(window, &color_table.colors).await;
    
        Self {
            renderer,
            camera,
            controller,

            delta_time: 0.0,
            last_frame_time: std::time::SystemTime::now(),

            world,
            particle_settings,
            color_table,

            followed_index: None,

            show_ui: true
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, scale_factor: f64) {
        if new_size.width > 0 && new_size.height > 0 {
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

        self.camera.zoom(self.controller.mouse_wheel * 0.025);

        if let Some(followed_index) = self.followed_index {
            self.camera.move_towards(3.0 * self.delta_time, self.world.get_particle(followed_index).position);
        } else {
            let camera_direction = glm::Vec2::new(
                self.controller.get_axis(Key::A, Key::D),
                self.controller.get_axis(Key::S, Key::W),
            );
    
            self.camera.move_xy(camera_direction * 400.0 * self.delta_time);
        }
        
        if self.controller.is_key_pressed(Key::R) {
            self.world.new_particles(self.color_table.colors.len(), self.particle_settings.max_particles);
        }
        if self.controller.is_key_pressed(Key::T) {
            self.color_table.new_table();
        }
        if self.controller.is_key_pressed(Key::Y) {
            self.show_ui = !self.show_ui;
        }

        self.world.update_partitions();

        self.world.update_particles(self.delta_time, &self.particle_settings, &self.color_table);

        if self.controller.is_button_pressed(Button::Left) && self.controller.is_key_down(Key::LShift) {
            let ndc: glm::Vec2 = glm::Vec2::new(
                self.controller.mouse_position.0 as f32 / self.camera.size.x * 2.0 - 1.0,
                (1.0 - self.controller.mouse_position.1 as f32 / self.camera.size.y) * 2.0 - 1.0
            );

            if let Some(id) = self.world.get_closest_particle_id(&self.camera.viewport_to_world(ndc)) {
                self.followed_index = Some(id);
                println!("Following: {}", self.followed_index.unwrap());
            }
        } else if self.controller.is_button_pressed(Button::Right) {
            self.followed_index = None;
        }

        self.controller.update();
    }

    fn render(&mut self, gui: &mut GUI) -> Result<(), wgpu::SurfaceError> {
        for particle in self.world.get_particles() {
            self.renderer.enqueue_instance(renderer::Instance {
                position: particle.position,
                color_id: particle.color_id as u32
            });
        }

        let frame_data = if self.show_ui {
            let mut max_r_changed = false;
            let mut colors_changed = false;

            let data = gui.draw_ui(
                &mut self.particle_settings,
                &mut self.color_table,
                &mut max_r_changed,
                &mut colors_changed,
                self.world.velocity_update_time,
                self.world.position_update_time,
                self.world.partition_update_time,
                self.renderer.gpu_time
            );

            if max_r_changed {
                self.world.new_partitions(2500.0, self.particle_settings.max_r);
            }
            if colors_changed {
                self.renderer.update_colors(&self.color_table.colors);
            }

            data
        } else {
            None
        };

        let result = self.renderer.render(
            self.camera.calc_matrices(),
            self.particle_settings.sharpness,
            self.particle_settings.radius,
            frame_data,
        );

        result
    }
}

async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1200, 1000))
        .with_title("Particle Life")
        .build(&event_loop)
        .unwrap();

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