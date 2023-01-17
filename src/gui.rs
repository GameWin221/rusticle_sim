use ::egui::FontDefinitions;
use egui_winit_platform::{Platform, PlatformDescriptor};
use egui::{FullOutput, ClippedPrimitive};

use winit::{window::Window, event::Event};

use crate::color_table::ColorTable;
use crate::particle_settings::{ParticleSettings, ParticleWrapping};

pub struct GUI {
    platform: Platform,
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
            platform
        }
    }

    pub fn handle_event(&mut self, event: &Event<()>) {
        self.platform.handle_event(event);
    }

    pub fn draw_ui(&mut self, 
        particle_settings: &mut ParticleSettings,
        color_table: &mut ColorTable,
        max_r_changed: &mut bool,
        colors_changed: &mut bool,
        velocity_update_time: f32,
        position_update_time: f32,
        partition_update_time: f32,
        gpu_time: f32,
    ) -> Option<(FullOutput, Vec<ClippedPrimitive>)> {
        self.platform.begin_frame();

        egui::Window::new(String::from("Particle Settings"))
            .default_width(420.0)
            .show(&self.platform.context(), |ui| {
                ui.heading("Physics");

                *max_r_changed = ui.add(egui::Slider::new(&mut particle_settings.max_r, particle_settings.min_r..=1000.0).text("Max r")).changed();
                ui.add(egui::Slider::new(&mut particle_settings.min_r, 0.0..=particle_settings.max_r).text("Min r"));
                ui.add(egui::Slider::new(&mut particle_settings.force, 0.0..=10.0).text("Force"));
                ui.add(egui::Slider::new(&mut particle_settings.drag, 0.0..=1.0).fixed_decimals(2).text("x^6 Drag"));

                if ui.button("Restore defaults").clicked() {
                    *particle_settings = ParticleSettings::default();
                    *max_r_changed = true;
                }

                ui.separator();

                ui.heading("Color Table");

                ui.horizontal(|ui| {
                    if ui.button("   Flip   ").clicked() {
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
                            *colors_changed = true;
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
                            *colors_changed = true;
                        }
                        
                        *color = glm::Vec3::from(rgb);

                        for value in row.iter_mut() {
                            //*value = 0.0;
                            ui.add(egui::DragValue::new(value).clamp_range(-1.0..=1.0).speed(0.1).fixed_decimals(2));
                        }
                    });
                }

                ui.separator();

                ui.heading("Rendering");

                ui.add(egui::Slider::new(&mut particle_settings.radius, 1.0..=60.0).text("Radius"));
                ui.add(egui::Slider::new(&mut particle_settings.sharpness, 0.0..=0.999).text("Sharpness"));
                //ui.add(egui::Slider::new(&mut particle_settings.drag, 0.0..=1.0).fixed_decimals(8).text("Drag"));

                ui.separator();

                ui.heading("Wrapping");

                ui.horizontal(|ui| {
                    ui.radio_value(&mut particle_settings.wrapping, ParticleWrapping::Barrier, "Barrier");
                    ui.radio_value(&mut particle_settings.wrapping, ParticleWrapping::Wrap, "Wrap");
                });
            });

        egui::Window::new(String::from("Metrics"))
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