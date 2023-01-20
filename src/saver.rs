use crate::color_table::ColorTable;
use crate::particle_settings::ParticleSettings;
use crate::world_settings::WorldSettings;

use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct ColorTableProxy {
    pub colors: Vec<[f32; 3]>,
    pub table: Vec<Vec<f32>>,
}

fn read_file(name: &String) -> std::io::Result<String> {
    let mut path = String::from("saved/");
    path.push_str(name.as_str());
    path.push_str(".json");

    let mut file = File::open(path)?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn save_file(data: &String, name: &String) -> std::io::Result<()> {
    let path_exists = std::path::Path::new("saved/").exists();

    if !path_exists {
        std::fs::create_dir("saved/").unwrap();
    }
    
    let mut path = String::from("saved/");
    path.push_str(name.as_str());
    path.push_str(".json");

    let mut file = File::create(path)?;

    file.write(data.as_bytes())?;

    Ok(())
}

pub fn save_particle_settings(particle_settings: &ParticleSettings, name: &String) -> std::io::Result<()> {
    let serialized = serde_json::to_string(&particle_settings)?;

    save_file(&serialized, name)?;

    Ok(())
}

pub fn read_particle_settings(name: &String) -> std::io::Result<ParticleSettings> {
    let serialized = read_file(name)?;

    let deserialized = serde_json::from_str(&serialized)?;

    Ok(deserialized)
}

pub fn save_color_table(color_table: &ColorTable, name: &String) -> std::io::Result<()> {
    let color_table_proxy = ColorTableProxy {
        colors: color_table.colors.iter().map(|&c| c.into()).collect(),
        table: color_table.table.clone(),
    };
    
    let serialized = serde_json::to_string(&color_table_proxy)?;

    save_file(&serialized, name)?;

    Ok(())
}

pub fn read_color_table(name: &String) -> std::io::Result<ColorTable> {
    let serialized = read_file(name)?;

    let deserialized: ColorTableProxy = serde_json::from_str(&serialized)?;

    let color_table = ColorTable {
        colors: deserialized.colors.iter().map(|&c| glm::Vec3::from(c)).collect(),
        table: deserialized.table.clone(),
    };

    Ok(color_table)
}

pub fn save_world_settings(world_settings: &WorldSettings, name: &String) -> std::io::Result<()> {
    let serialized = serde_json::to_string(&world_settings)?;

    save_file(&serialized, name)?;

    Ok(())
}

pub fn read_world_settings(name: &String) -> std::io::Result<WorldSettings> {
    let serialized = read_file(name)?;

    let deserialized = serde_json::from_str(&serialized)?;

    Ok(deserialized)
}