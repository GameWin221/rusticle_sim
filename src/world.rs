extern crate nalgebra_glm as glm;

use rand::Rng;
use rayon::prelude::{ParallelIterator, IndexedParallelIterator, IntoParallelRefIterator};

use crate::{
    particle::Particle,
    particle_settings::{ParticleSettings, ParticleWrapping}
};

use std::{
    sync::{Arc, RwLock},
    f32::consts::PI, time::Instant
};

const DEFAULT_NUM_PARTICLES_PER_CELL: usize = 256;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct PartitionCell {
    pub particles: Vec<usize>,
}

impl PartitionCell {
    pub fn new() -> Self {
        Self {
            particles: Vec::with_capacity(DEFAULT_NUM_PARTICLES_PER_CELL),
        }
    }
}

pub struct World {
    particles: Arc<RwLock<Vec<Particle>>>,
    partitions: Arc<RwLock<Vec<PartitionCell>>>,

    size: f32,
    cell_size: f32, 
    cell_count: usize,

    max_particles: usize,
}

impl World {
    pub fn new(size: f32, cell_size: f32, max_particles: usize) -> Self {
        let cell_count = (size * 2.0 / cell_size).ceil() as usize;

        println!("Creating a world with size: {}x{} and {}x{} partitions (each {}x{})", size*2.0, size*2.0, cell_count, cell_count, cell_size, cell_size);

        Self {
            particles: Arc::new(RwLock::new(Vec::with_capacity(max_particles))),
            partitions: Arc::new(RwLock::new(vec![PartitionCell::new(); cell_count*cell_count])), 

            size,
            cell_size,
            cell_count,

            max_particles
        }
    }

    pub fn get_particles(&self) -> Vec<Particle> {
        self.particles.read().unwrap().iter().map(|p| {
            p.clone()
        }).collect()
    }

    pub fn gen_particles(&mut self, color_count: usize) {
        self.particles = Arc::new(RwLock::new((0..self.max_particles).map(|_| {
            Particle::new(
                glm::Vec2::new(
                    rand::thread_rng().gen_range(-self.size ..=self.size ),
                    rand::thread_rng().gen_range(-self.size..=self.size)
                ), 
                rand::thread_rng().gen_range(0..color_count as u8)
            )
        }).collect()));
    }

    pub fn update_particles(&mut self, delta_time: f32, particle_settings: &ParticleSettings) {
        let start = std::time::Instant::now();

        let duration = Arc::new(RwLock::new(std::time::Duration::ZERO));

        {
        let all_partitions = self.partitions.read().unwrap();
        let all_particles = self.particles.read().unwrap().clone();

        let particle_speed = 75.0 * particle_settings.force * delta_time;

        all_partitions.par_iter().enumerate().for_each(|(index, partition)|{
            let mut other_partitions = Vec::with_capacity(9);
            other_partitions.push(partition);
            
            let y_i = index / self.cell_count;
            let x_i = index - y_i * self.cell_count;

            match particle_settings.wrapping {
                ParticleWrapping::Barrier => {
                    if x_i >= 1 { // Left
                        other_partitions.push(&all_partitions[index-1]); 

                        if y_i >= 1 { // Bottom Left
                            other_partitions.push(&all_partitions[index-self.cell_count-1]);  
                        }
                        if y_i < self.cell_count - 1 { // Top Left
                            other_partitions.push(&all_partitions[index+self.cell_count-1]);  
                        }
                    }
                    if x_i < self.cell_count - 1 { // Right
                        other_partitions.push(&all_partitions[index+1]); 

                        if y_i >= 1 { // Bottom Right
                            other_partitions.push(&all_partitions[index-self.cell_count+1]);  
                        }
                        if y_i < self.cell_count - 1 { // Top Right
                            other_partitions.push(&all_partitions[index+self.cell_count+1]);  
                        }
                    }
                    if y_i >= 1 { // Bottom
                        other_partitions.push(&all_partitions[index-self.cell_count]);  
                    }
                    if y_i < self.cell_count - 1 { // Top
                        other_partitions.push(&all_partitions[index+self.cell_count]);  
                    }
                }
                ParticleWrapping::Wrap => {
                    /*
                    fn to_id(x: usize, y: usize, w: usize) -> usize {
                        y * w + x
                    }
                    fn wrap(i: i32, max: usize) -> usize {
                        if i < 0 {
                            (max as i32 + i) as usize
                        }
                        else if i as usize > max {
                            i as usize - max
                        } else {
                            i as usize
                        }
                    }

                    let y = index / self.cell_count;
                    let x = index - y * self.cell_count;
                    let w = self.cell_count-1;

                    
                    println!("Partition:\nx: {}, y: {}\nx: {}, y: {}\nx: {}, y: {}\nx: {}, y: {}\nx: {}, y: {}\nx: {}, y: {}\nx: {}, y: {}\nx: {}, y: {}\nx: {}, y: {}",
                        x, y,
                        wrap(x as i32 - 1, w), y,
                        wrap(x as i32 - 1, w), wrap(y as i32 - 1, w),
                        wrap(x as i32 - 1, w), wrap(y as i32 + 1, w),
                        wrap(x as i32 + 1, w), y,
                        wrap(x as i32 + 1, w), wrap(y as i32 - 1, w),
                        wrap(x as i32 + 1, w), wrap(y as i32 + 1, w),
                        x, wrap(y as i32 + 1, w),
                        x, wrap(y as i32 - 1, w),
                    );
                    */
                    /*
                    // Left
                    other_partitions.push(&all_partitions[to_id(wrap(x as i32 - 1, w), y, w)]); 
                    // Bottom Left
                    other_partitions.push(&all_partitions[to_id(wrap(x as i32 - 1, w), wrap(y as i32 - 1, w), w)]);  
                    // Top Left
                    other_partitions.push(&all_partitions[to_id(wrap(x as i32 - 1, w), wrap(y as i32 + 1, w), w)]);  

                    // Right
                    other_partitions.push(&all_partitions[to_id(wrap(x as i32 + 1, w), y, w)]); 
                    // Bottom Right
                    other_partitions.push(&all_partitions[to_id(wrap(x as i32 + 1, w), wrap(y as i32 - 1, w), w)]);  
                    // Top Right
                    other_partitions.push(&all_partitions[to_id(wrap(x as i32 + 1, w), wrap(y as i32 + 1, w), w)]);  

                    // Bottom
                    other_partitions.push(&all_partitions[to_id(x, wrap(y as i32 + 1, w), w)]);  
                    // Top
                    other_partitions.push(&all_partitions[to_id(x, wrap(y as i32 - 1, w), w)]);  
                    */
                }
            }

            for &index in &partition.particles {
                let mut new_particle = all_particles[index].clone();

                new_particle.velocity *= particle_settings.drag.powf(delta_time);

                for &other_partition in &other_partitions {
                    for &other_index in &other_partition.particles {
                        let other = &all_particles[other_index];

                        let diff: glm::Vec2 = other.position - new_particle.position;
                        let dist: f32 = (diff.x*diff.x+diff.y*diff.y).sqrt();
        
                        if dist < 0.0001 || dist > particle_settings.max_r {
                            continue;
                        } 
        
                        let dir: glm::Vec2 = diff / dist;
                        let particle_acceleration: glm::Vec2 = particle_speed * dir;
                        
                        // https://www.desmos.com/calculator/yacrclthei?lang=pl
                        if dist > particle_settings.min_r {
                            let c = particle_settings.color_table[new_particle.color_id as usize][other.color_id as usize];
                            new_particle.velocity += particle_acceleration * c * ((PI*(dist - particle_settings.min_r)) / (particle_settings.max_r - particle_settings.min_r)).sin();
                        } else {
                            new_particle.velocity += particle_acceleration * (dist / particle_settings.min_r - 1.0);
                        }
                    }
                }

                self.particles.write().unwrap()[index] = new_particle;
            }
        });
        }

        self.particles.write().unwrap().iter_mut().for_each(|particle| {
            particle.position += particle.velocity * delta_time;

            match particle_settings.wrapping {
                ParticleWrapping::Wrap => {
                    if particle.position.x >= self.size-0.1 {
                        particle.position.x = -self.size+0.1;
                    } if particle.position.x <= -self.size+0.1 {
                        particle.position.x = self.size-0.1;
                    }

                    if particle.position.y >= self.size-0.1 {
                        particle.position.y = -self.size+0.1;
                    } if particle.position.y <= -self.size+0.1 {
                        particle.position.y = self.size-0.1;
                    }
                }
                ParticleWrapping::Barrier => {
                    particle.position.x = particle.position.x.clamp(-self.size+0.1, self.size-0.1);
                    particle.position.y = particle.position.y.clamp(-self.size+0.1, self.size-0.1);
                }
            }
        });

        let elapsed = start.elapsed();

        print!("Physics update took: {:.2}ms) ", elapsed.as_secs_f32()*1000.0);
    }

    pub fn gen_partitions(&mut self, world_size: f32, cell_size: f32) {
        let cell_count = (world_size * 2.0 / cell_size).ceil() as usize;

        self.size = world_size;
        self.partitions = Arc::new(RwLock::new(vec![PartitionCell::new(); cell_count*cell_count]));
        self.cell_count = cell_count;
        self.cell_size = cell_size;

        println!("Creating a world with size: {}x{} and {}x{} partitions (each {}x{})", self.size*2.0, self.size*2.0, self.cell_count, self.cell_count, cell_size, cell_size);
    }

    pub fn update_partitions(&mut self) {
        let start = std::time::Instant::now();

        let partitions = &mut self.partitions.write().unwrap();

        partitions.iter_mut().for_each(|partition|{
            partition.particles.clear();
        });

        self.particles.write().unwrap().iter().enumerate().for_each(|(index, particle)| {
            let id = self.get_partition_id(&particle.position);

            partitions[id].particles.push(index);
        });

        let update = start.elapsed().as_secs_f64()*1000.0;

        print!("Partition update took: {:.2}ms) ", update);
    }

    fn get_partition_id(&self, pos: &glm::Vec2) -> usize {
        let (x, y) = (
            ((pos.x + self.size) / self.cell_size).floor() as usize, 
            ((pos.y + self.size) / self.cell_size).floor() as usize
        );

        y * self.cell_count + x
    }
}