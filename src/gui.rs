use ::egui::FontDefinitions;
use egui_winit_platform::{Platform, PlatformDescriptor};
use egui::{FullOutput, ClippedPrimitive};

use winit::{window::Window, event::Event};

use crate::color_table::ColorTable;
use crate::particle_settings::ParticleSettings;
use crate::renderer::MAX_INSTANCES;
use crate::saver;
use crate::world_settings::{WorldSettings, ParticleWrapping};

pub struct GUI {
    platform: Platform,

    world_settings_name: String,
    color_table_name: String,
    particle_settings_name: String,

    world_settings_failed: bool,
    color_table_failed: bool,
    particle_settings_failed: bool,
}

impl GUI {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.width as u32,
            physical_height: size.height as u32,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        Self {
            platform,

            world_settings_name: String::from("Save Name"),
            color_table_name: String::from("Save Name"),
            particle_settings_name: String::from("Save Name"),

            world_settings_failed: false,
            color_table_failed: false,
            particle_settings_failed: false,
        }
    }

    pub fn handle_event(&mut self, event: &Event<()>) {
        self.platform.handle_event(event);
    }

    pub fn draw_ui(&mut self, 
        world_settings: &mut WorldSettings,
        particle_settings: &mut ParticleSettings,
        color_table: &mut ColorTable,
        fixed_time_step: &mut bool,
        should_update_world: &mut bool,
        should_update_particles: &mut bool,
        should_update_colors: &mut bool,
        simulate: &mut bool,
        time_step: &mut f32,
        velocity_update_time: f32,
        position_update_time: f32,
        partition_update_time: f32,
        gpu_time: f32,
    ) -> Option<(FullOutput, Vec<ClippedPrimitive>)> {
        self.platform.begin_frame();

        egui::Window::new(String::from("Control Panel"))
            .anchor(egui::Align2::LEFT_TOP, [5.0, 5.0])
            .show(&self.platform.context(), |ui| {
                ui.heading("Quick Access");

                *should_update_particles = ui.button("Respawn Particles").clicked();
                if ui.button("Respawn Particles + Random Seed").clicked() {
                    world_settings.new_random_seed();
                    *should_update_particles = true;
                }
                if ui.button("Randomize Color Table").clicked() {
                    color_table.new_table();
                }

                ui.separator();
                
                ui.checkbox(simulate, "Simulate");

                ui.checkbox(fixed_time_step, "Fixed time step");
                ui.label("Will stabilise the simulation especially at lower FPS");

                if *fixed_time_step {
                    ui.add(egui::Slider::new(time_step, 0.0..=0.1).text("[s/tick] Time Step").fixed_decimals(3).step_by(0.002));
                }

                ui.separator();

                ui.collapsing("World Settings", |ui| {
                    ui.label("Seed:");

                    ui.text_edit_singleline(&mut world_settings.seed);

                    if ui.button("Randomize").clicked() {
                        world_settings.new_random_seed();
                    }

                    ui.separator();
                    
                    ui.label("Size:");
                    ui.horizontal(|ui| {
                        if ui.add(egui::DragValue::new(&mut world_settings.size).clamp_range(1000.0..=10000.0)).changed() {
                            *should_update_world = true;
                        }
                        ui.label("x");
    
                        if ui.add(egui::DragValue::new(&mut world_settings.size).clamp_range(1000.0..=10000.0)).changed() {
                            *should_update_world = true;
                        }
                    });

                    ui.separator();
                    
                    ui.label("Background Color:");
                    ui.color_edit_button_rgb(&mut world_settings.bg_color);

                    ui.separator();
                    ui.label("Particle Count:");
                    if ui.add(egui::Slider::new(&mut world_settings.max_particles, 0..=MAX_INSTANCES)).changed() {
                        *should_update_particles = true;
                    }
                    ui.separator();

                    ui.label("Wrapping:");
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut world_settings.wrapping, ParticleWrapping::Barrier, "Barrier");
                        ui.radio_value(&mut world_settings.wrapping, ParticleWrapping::Wrap, "Wrap");
                    });

                    ui.separator();

                    if ui.button("Restore defaults").clicked() {
                        *world_settings = WorldSettings::default();
                        *should_update_world = true;
                        *should_update_particles = true;
                    }

                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            saver::save_world_settings(world_settings, &self.world_settings_name).unwrap();
                        } 
                        if ui.button("Load").clicked() {
                            if let Ok(new_world_settings) = saver::read_world_settings(&self.world_settings_name) {
                                *world_settings = new_world_settings;
                                *should_update_world = true;
                                *should_update_particles = true;
                                self.world_settings_failed = false;
                            } else {
                                self.world_settings_failed = true;
                            }
                        }

                        ui.text_edit_singleline(&mut self.world_settings_name);
                    });

                    if self.world_settings_failed {
                        ui.label("Failed to load world settings!");
                    }
                });

                ui.separator();

                ui.collapsing("Particle settings", |ui| {
                    *should_update_world = ui.add(egui::Slider::new(&mut particle_settings.max_r, particle_settings.min_r+0.1..=1000.0).text("Max influence radius")).changed();
                    ui.add(egui::Slider::new(&mut particle_settings.min_r, 10.0..=particle_settings.max_r-0.1).text("Min influence radius"));
                    ui.add(egui::Slider::new(&mut particle_settings.force, 0.0..=10.0).text("Atraction force"));
                    ui.add(egui::Slider::new(&mut particle_settings.drag, 0.0..=1.0).fixed_decimals(2).text("Velocity over time"));
    
                    ui.separator();

                    if ui.button("Restore defaults").clicked() {
                        *particle_settings = ParticleSettings::default();
                        *should_update_world = true;
                    }

                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            saver::save_particle_settings(particle_settings, &self.particle_settings_name).unwrap();
                        } 
                        if ui.button("Load").clicked() {
                            if let Ok(new_particle_settings) = saver::read_particle_settings(&self.particle_settings_name) {
                                *particle_settings = new_particle_settings;
                                *should_update_world = true;
                                self.particle_settings_failed = false;
                            } else {
                                self.particle_settings_failed = true;
                            }
                        }

                        ui.text_edit_singleline(&mut self.particle_settings_name);
                    });

                    if self.particle_settings_failed {
                        ui.label("Failed to load particle settings!");
                    }
                });

                ui.separator();

                ui.collapsing("Color Table", |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("  Flip   ").clicked() {
                            let color_count = color_table.colors.len();
    
                            for y in 0..color_count {
                                for x in 0..=y {
                                    let tmp = color_table.table[y][x];
                                    color_table.table[y][x] = color_table.table[color_count-1-y][color_count-1-x];
                                    color_table.table[color_count-1-y][color_count-1-x] = tmp;
                                }
                            }
                        }
    
                        for (y_index, _) in color_table.table.iter_mut().enumerate() {
                        
                            let color = &mut color_table.colors[y_index];
    
                            let mut rgb = [color.x, color.y, color.z];
    
                            if ui.color_edit_button_rgb(&mut rgb).changed() {
                                *should_update_colors = true;
                            }
                            
                            *color = glm::Vec3::from(rgb);
    
                            continue;
                        }
                    });
                    for (y_index, row) in color_table.table.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            let color = &mut color_table.colors[y_index];
    
                            let mut rgb = [color.x, color.y, color.z];
    
                            if ui.color_edit_button_rgb(&mut rgb).changed() {
                                *should_update_colors = true;
                            }
                            
                            *color = glm::Vec3::from(rgb);
    
                            for value in row.iter_mut() {
                                ui.add(egui::DragValue::new(value).clamp_range(-1.0..=1.0).speed(0.1).fixed_decimals(2));
                                if *value < 0.0 {
                                    ui.add_space(-3.0);
                                }
                            }
                        });
                    }
                
                    ui.separator();

                    ui.horizontal(|ui|{
                        if ui.button("Add Color").clicked() {
                            color_table.add_color();
                            *should_update_colors = true;
                        }
    
                        if ui.button("Remove Color").clicked() {
                            color_table.remove_color();
                            *should_update_colors = true;
                        }
                    });

                    ui.separator();

                    if ui.button("Randomize").clicked() {
                        color_table.new_table();
                    }

                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            saver::save_color_table(color_table, &self.color_table_name).unwrap();
                        } 
                        if ui.button("Load").clicked() {
                            if let Ok(new_color_table) = saver::read_color_table(&self.color_table_name) {
                                *color_table = new_color_table;
                                *should_update_colors = true;
                                self.color_table_failed = false;
                            } else {
                                self.color_table_failed = true;
                            }
                        }

                        ui.text_edit_singleline(&mut self.color_table_name);
                    });

                    if self.color_table_failed {
                        ui.label("Failed to load color table!");
                    }
                });

                ui.separator();

                ui.collapsing("Rendering", |ui| {
                    ui.add(egui::Slider::new(&mut particle_settings.radius, 1.0..=60.0).text("Particle Radius"));
                    ui.add(egui::Slider::new(&mut particle_settings.sharpness, 0.0..=0.999).text("Particle Sharpness"));
                });

                ui.separator();

                ui.collapsing("Keybindings", |ui| {
                    ui.label("Shift + U - Toggle UI");
                    ui.label("Shift + LMB - Follow a particle");
                    ui.label("Shift + RMB - Stop following");
                });
            });

        egui::Window::new(String::from("Metrics"))
            .anchor(egui::Align2::RIGHT_TOP, [-5.0, 5.0])
            .show(&self.platform.context(), |ui| {
                ui.label(format!("Velocity update time: {:.2}ms", velocity_update_time));
                ui.label(format!("Position update time: {:.2}ms", position_update_time));
                ui.label(format!("Partition update time: {:.2}ms", partition_update_time));
                ui.label(format!("GPU time: {:.2}ms", gpu_time));
            });
    
        let full_output = self.platform.end_frame(None);
        let paint_jobs = self.platform.context().tessellate(full_output.shapes.clone());
    
        Some((full_output, paint_jobs))
    }
}