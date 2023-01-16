extern crate nalgebra_glm as glm;

use rand::Rng;
use rayon::prelude::{ParallelIterator, IndexedParallelIterator, IntoParallelRefIterator};

use crate::{
    particle::Particle,
    particle_settings::{ParticleSettings, ParticleWrapping}
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
    particles: Vec<Particle>,
    partitions: Vec<PartitionCell>,

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
            particles: Vec::with_capacity(max_particles),
            partitions: vec![PartitionCell::new(); cell_count*cell_count], 

            size,
            cell_size,
            cell_count,

            max_particles
        }
    }

    pub fn get_particles(&self) -> Vec<Particle> {
        self.particles.iter().map(|p| {
            p.clone()
        }).collect()
    }

    pub fn gen_particles(&mut self, color_count: usize) {
        self.particles = (0..self.max_particles).map(|_| {
            Particle::new(
                glm::Vec2::new(
                    rand::thread_rng().gen_range(-self.size ..=self.size ),
                    rand::thread_rng().gen_range(-self.size..=self.size)
                ), 
                rand::thread_rng().gen_range(0..color_count as u8)
            )
        }).collect();
    }

    pub fn update_particles(&mut self, delta_time: f32, particle_settings: &ParticleSettings) {
        let particles_vec_ptr: *const Vec<Particle> = &self.particles;
        let particles_vec_addr = particles_vec_ptr as usize;

        let particle_speed = 75.0 * particle_settings.force * delta_time;

        let min_r_norm = particle_settings.min_r / particle_settings.max_r;
 
        let start = std::time::Instant::now();
        self.partitions.par_iter().enumerate().for_each(|(index, partition)|{
            unsafe {
            let particles_mut = &mut*(particles_vec_addr as *mut Vec<Particle>);

            let mut other_partitions = Vec::with_capacity(9);
            other_partitions.push(partition);
            
            let y_i = index / self.cell_count;
            let x_i = index - y_i * self.cell_count;

            match particle_settings.wrapping {
                ParticleWrapping::Barrier => {
                    if x_i >= 1 { // Left
                        other_partitions.push(&self.partitions[index-1]); 

                        if y_i >= 1 { // Bottom Left
                            other_partitions.push(&self.partitions[index-self.cell_count-1]);  
                        }
                        if y_i < self.cell_count - 1 { // Top Left
                            other_partitions.push(&self.partitions[index+self.cell_count-1]);  
                        }
                    }
                    if x_i < self.cell_count - 1 { // Right
                        other_partitions.push(&self.partitions[index+1]); 

                        if y_i >= 1 { // Bottom Right
                            other_partitions.push(&self.partitions[index-self.cell_count+1]);  
                        }
                        if y_i < self.cell_count - 1 { // Top Right
                            other_partitions.push(&self.partitions[index+self.cell_count+1]);  
                        }
                    }
                    if y_i >= 1 { // Bottom
                        other_partitions.push(&self.partitions[index-self.cell_count]);  
                    }
                    if y_i < self.cell_count - 1 { // Top
                        other_partitions.push(&self.partitions[index+self.cell_count]);  
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
                    other_partitions.push(&self.partitions[to_id(wrap(x as i32 - 1, w), y, w)]); 
                    // Bottom Left
                    other_partitions.push(&self.partitions[to_id(wrap(x as i32 - 1, w), wrap(y as i32 - 1, w), w)]);  
                    // Top Left
                    other_partitions.push(&self.partitions[to_id(wrap(x as i32 - 1, w), wrap(y as i32 + 1, w), w)]);  

                    // Right
                    other_partitions.push(&self.partitions[to_id(wrap(x as i32 + 1, w), y, w)]); 
                    // Bottom Right
                    other_partitions.push(&self.partitions[to_id(wrap(x as i32 + 1, w), wrap(y as i32 - 1, w), w)]);  
                    // Top Right
                    other_partitions.push(&self.partitions[to_id(wrap(x as i32 + 1, w), wrap(y as i32 + 1, w), w)]);  

                    // Bottom
                    other_partitions.push(&self.partitions[to_id(x, wrap(y as i32 + 1, w), w)]);  
                    // Top
                    other_partitions.push(&self.partitions[to_id(x, wrap(y as i32 - 1, w), w)]);  
                    */
                }
            }

            // Maybe this approach would be better? https://stackoverflow.com/questions/71182117/change-elements-in-vector-using-multithreading-in-rust
            // Split the workload (particle vector) to smaller vectors and move them to their respective threads
            // After the work is done, retrieve the particles and copy them to the original vector
            // 'crossbeam' crate will be really helpful with its scoped threads

            for &index in &partition.particles {
                let particle = &mut particles_mut[index];

                particle.velocity *= particle_settings.drag.powf(delta_time);

                for &other_partition in &other_partitions{
                    for &other_index in &other_partition.particles {
                        let other = &self.particles[other_index];

                        let diff: glm::Vec2 = other.position - particle.position;
                        let dist: f32 = (diff.x*diff.x+diff.y*diff.y).sqrt();
    
                        if dist == 0.0 {
                            continue;
                        } 

                        let particle_acceleration: glm::Vec2 = diff / dist * particle_speed;
                        
                        // https://www.desmos.com/calculator/xjmwts0q8l
                        if dist > particle_settings.min_r {
                            let c = particle_settings.color_table[particle.color_id as usize][other.color_id as usize];
                            // Old equation was removed because it was too costly: ((PI*(dist - particle_settings.min_r)) / (particle_settings.max_r - particle_settings.min_r)).sin();
                            let v = 1.0 - (1.0 + min_r_norm - 2.0 * (dist/particle_settings.max_r).min(1.0)).abs() / (1.0 - min_r_norm);
                            
                            particle.velocity += particle_acceleration * c * v;
                        } else {
                            particle.velocity += particle_acceleration * (dist / particle_settings.min_r - 1.0);
                        }
                    }
                }
            }}
        });

        let elapsed = start.elapsed();

        self.particles.iter_mut().for_each(|particle| {
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



        print!("Physics update took: {:.2}ms) ", elapsed.as_secs_f32()*1000.0);
    }

    pub fn gen_partitions(&mut self, world_size: f32, cell_size: f32) {
        let cell_count = (world_size * 2.0 / cell_size).ceil() as usize;

        self.size = world_size;
        self.partitions = vec![PartitionCell::new(); cell_count*cell_count];
        self.cell_count = cell_count;
        self.cell_size = cell_size;

        println!("Creating a world with size: {}x{} and {}x{} partitions (each {}x{})", self.size*2.0, self.size*2.0, self.cell_count, self.cell_count, cell_size, cell_size);
    }

    pub fn update_partitions(&mut self) {
        let start = std::time::Instant::now();

        self.partitions.iter_mut().for_each(|partition|{
            partition.particles.clear();
        });
    
        let particles = self.particles.clone();

        particles.iter().enumerate().for_each(|(index, particle)| {
            let id = self.get_partition_id(&particle.position);

            self.partitions[id].particles.push(index);
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