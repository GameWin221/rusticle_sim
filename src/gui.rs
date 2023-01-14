extern crate nalgebra_glm as glm;

use ::egui::FontDefinitions;
use egui_winit_platform::{Platform, PlatformDescriptor};
use egui::{FullOutput, ClippedPrimitive};

use winit::{window::Window, event::Event};

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

    pub fn draw_ui(&mut self, particle_settings: &mut ParticleSettings) -> Option<(FullOutput, Vec<ClippedPrimitive>)> {
        self.platform.begin_frame();

        egui::Window::new(String::from("Particle Settings"))
            .default_width(420.0)
            .show(&self.platform.context(), |ui| {
                /*
                for color in &mut particle_settings.colors {
                    let mut c = [color.x, color.y, color.z]; 

                    ui.color_edit_button_rgb(&mut c);

                    color.x = c[0];
                    color.y = c[1];
                    color.z = c[2];
                }
                */

                ui.heading("Physics");

                ui.add(egui::Slider::new(&mut particle_settings.max_r, particle_settings.min_r..=1000.0).text("Max r"));
                ui.add(egui::Slider::new(&mut particle_settings.min_r, 0.0..=particle_settings.max_r).text("Min r"));
                ui.add(egui::Slider::new(&mut particle_settings.force, 0.0..=10.0).text("Force"));
                ui.add(egui::Slider::new(&mut particle_settings.radius, 1.0..=50.0).text("Radius"));
                //ui.add(egui::Slider::new(&mut particle_settings.drag, 0.0..=1.0).fixed_decimals(8).text("Drag"));

                ui.separator();

                ui.heading("Wrapping");

                ui.horizontal(|ui| {
                    ui.radio_value(&mut particle_settings.wrapping, ParticleWrapping::None, "None");
                    ui.radio_value(&mut particle_settings.wrapping, ParticleWrapping::Barrier, "Barrier");
                    ui.radio_value(&mut particle_settings.wrapping, ParticleWrapping::Wrap, "Wrap");
                });
            });
    
        let full_output = self.platform.end_frame(None);
        let paint_jobs = self.platform.context().tessellate(full_output.shapes.clone());
    
        Some((full_output, paint_jobs))
    }
}