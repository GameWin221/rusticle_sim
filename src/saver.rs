use crate::color_table::ColorTable;
use crate::particle_settings::ParticleSettings;

use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct ColorTableProxy {
    pub colors: Vec<[f32; 3]>,
    pub table: Vec<Vec<f32>>,
}

pub fn save_particle_settings(particle_settings: &ParticleSettings, name: String) -> std::io::Result<()> {
    let serialized = serde_json::to_string(&particle_settings)?;

    let mut path = String::from("saved/");
    path.push_str(name.as_str());
    path.push_str(".json");

    let mut file = File::create(path)?;

    file.write(serialized.as_bytes())?;

    Ok(())
}

pub fn read_particle_settings(name: String) -> std::io::Result<ParticleSettings> {
    let mut path = String::from("saved/");
    path.push_str(name.as_str());
    path.push_str(".json");

    let mut file = File::open(path)?;

    let mut serialized = String::new();

    file.read_to_string(&mut serialized)?;

    let deserialized = serde_json::from_str(&serialized)?;

    Ok(deserialized)
}

pub fn save_color_table(color_table: &ColorTable, name: String) -> std::io::Result<()> {
    let color_table_proxy = ColorTableProxy {
        colors: color_table.colors.iter().map(|&c| c.into()).collect(),
        table: color_table.table.clone(),
    };
    
    let serialized = serde_json::to_string(&color_table_proxy)?;

    let mut path = String::from("saved/");
    path.push_str(name.as_str());
    path.push_str(".json");

    let mut file = File::create(path)?;

    file.write(serialized.as_bytes())?;

    Ok(())
}

pub fn read_color_table(name: String) -> std::io::Result<ColorTable> {
    let mut path = String::from("saved/");
    path.push_str(name.as_str());
    path.push_str(".json");

    let mut file = File::open(path)?;

    let mut serialized = String::new();

    file.read_to_string(&mut serialized)?;

    let deserialized: ColorTableProxy = serde_json::from_str(&serialized)?;

    let color_table = ColorTable {
        colors: deserialized.colors.iter().map(|&c| glm::Vec3::from(c)).collect(),
        table: deserialized.table.clone(),
    };

    Ok(color_table)
}