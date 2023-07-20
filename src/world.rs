use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use rayon::prelude::{ParallelIterator, IndexedParallelIterator, IntoParallelRefIterator};

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    particle_settings::ParticleSettings,
    world_settings::{WorldSettings, ParticleWrapping},
    color_table::ColorTable
};

const DEFAULT_NUM_PARTICLES_PER_CELL: usize = 256;
const BARRIER_MARGIN: f32 = 0.1;

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
    particle_positions: Vec<glm::Vec2>,
    particle_velocities: Vec<glm::Vec2>,
    particle_color_ids: Vec<u8>,
    partitions: Vec<PartitionCell>,

    size: f32,
    half_size: f32,
    cell_size: f32, 
    cell_count: usize,

    pub velocity_update_time: f32,
    pub position_update_time: f32,
    pub partition_update_time: f32,
}

impl World {
    pub fn new(world_settings: &WorldSettings, particle_settings: &ParticleSettings) -> Self {
        let size = world_settings.size;
        let half_size = size / 2.0;
        let cell_size = particle_settings.max_r;
        let cell_count = (size / cell_size).ceil() as usize;

        Self {
            particle_positions: Vec::new(),
            particle_velocities: Vec::new(),
            particle_color_ids: Vec::new(),
            partitions: vec![PartitionCell::new(); cell_count*cell_count], 

            size,
            half_size,
            cell_size,
            cell_count,

            velocity_update_time: 0.0,
            position_update_time: 0.0,
            partition_update_time: 0.0,
        }
    }

    pub fn get_particle_positions(&self) -> &Vec<glm::Vec2> {
        &self.particle_positions
    }
    pub fn get_particle_color_ids(&self) -> &Vec<u8> {
        &self.particle_color_ids
    }

    pub fn get_particle_position(&self, index: usize) -> glm::Vec2 {
        *self.particle_positions.get(index).unwrap()
    }
    pub fn get_particle_color_id(&self, index: usize) -> u8 {
        *self.particle_color_ids.get(index).unwrap()
    }

    pub fn new_particles(&mut self, world_settings: &WorldSettings, color_table: &ColorTable) {
        let mut hasher = DefaultHasher::new();
        world_settings.seed.hash(&mut hasher);
        
        let seed = hasher.finish();

        let mut r = StdRng::seed_from_u64(seed);
        
        self.particle_positions = (0..world_settings.max_particles).map(|_| {
            glm::Vec2::new(
                r.gen_range(-self.half_size+BARRIER_MARGIN..=self.half_size-BARRIER_MARGIN),
                r.gen_range(-self.half_size+BARRIER_MARGIN..=self.half_size-BARRIER_MARGIN)
            )
        }).collect();
        self.particle_velocities = (0..world_settings.max_particles).map(|_| {
            glm::Vec2::zeros()
        }).collect();
        self.particle_color_ids = (0..world_settings.max_particles).map(|_| {
            r.gen_range(0..color_table.colors.len() as u8)
        }).collect();
    }

    pub fn clamp_particle_colors(&mut self, color_table: &ColorTable) {
        self.particle_color_ids.iter_mut().for_each(|color_id|{
            *color_id = (*color_id).min((color_table.colors.len() - 1) as u8);
        })
    }

    pub fn update_particles(&mut self, delta_time: f32, particle_settings: &ParticleSettings, world_settings: &WorldSettings, color_table: &ColorTable) {
        let start = std::time::Instant::now();

        let particle_positions_vec_ptr: *const Vec<glm::Vec2> = &self.particle_positions;
        let particle_positions_vec_addr = particle_positions_vec_ptr as usize;

        let particle_velocities_vec_ptr: *const Vec<glm::Vec2> = &self.particle_velocities;
        let particle_velocities_vec_addr = particle_velocities_vec_ptr as usize;

        let particle_color_ids_vec_ptr: *const Vec<u8> = &self.particle_color_ids;
        let particle_color_ids_vec_addr = particle_color_ids_vec_ptr as usize;

        let particle_speed = 75.0 * particle_settings.force * delta_time;

        let drag = particle_settings.drag.powi(6);

        let min_r_norm = particle_settings.min_r / particle_settings.max_r;
 
        // par_iter() from rayon
        self.partitions.par_iter().enumerate().for_each(|(index, partition)|{
            let particle_positions_mut = unsafe { &mut*(particle_positions_vec_addr as *mut Vec<glm::Vec2>) }; // Shhh
            let particle_velocities_mut = unsafe { &mut*(particle_velocities_vec_addr as *mut Vec<glm::Vec2>) }; // Shhh
            let particle_color_ids_mut = unsafe { &mut*(particle_color_ids_vec_addr as *mut Vec<u8>) }; // Shhh

            let mut other_partitions = Vec::with_capacity(9);
            other_partitions.push((partition, glm::Vec2::zeros()));
            
            let y_i = index / self.cell_count;
            let x_i = index - y_i * self.cell_count;

            let w = self.cell_count;
            let w_max = w-1;

            // Get neighbors
            match world_settings.wrapping {
                ParticleWrapping::Barrier => {
                    if x_i >= 1 { // Left
                        other_partitions.push((&self.partitions[index-1], glm::Vec2::zeros())); 

                        if y_i >= 1 { // Bottom Left
                            other_partitions.push((&self.partitions[index-self.cell_count-1], glm::Vec2::zeros()));  
                        }
                        if y_i < self.cell_count - 1 { // Top Left
                            other_partitions.push((&self.partitions[index+self.cell_count-1], glm::Vec2::zeros()));  
                        }
                    }
                    if x_i < self.cell_count - 1 { // Right
                        other_partitions.push((&self.partitions[index+1], glm::Vec2::zeros())); 

                        if y_i >= 1 { // Bottom Right
                            other_partitions.push((&self.partitions[index-self.cell_count+1], glm::Vec2::zeros()));  
                        }
                        if y_i < self.cell_count - 1 { // Top Right
                            other_partitions.push((&self.partitions[index+self.cell_count+1], glm::Vec2::zeros()));  
                        }
                    }
                    if y_i >= 1 { // Bottom
                        other_partitions.push((&self.partitions[index-self.cell_count], glm::Vec2::zeros()));  
                    }
                    if y_i < self.cell_count - 1 { // Top
                        other_partitions.push((&self.partitions[index+self.cell_count], glm::Vec2::zeros()));  
                    }
                }
                ParticleWrapping::Wrap => {
                    fn to_id(x: usize, y: usize, w: usize) -> usize {
                        y * w + x
                    }
                    fn wrap(i: i32, max: usize) -> (usize, bool) {
                        if i < 0 {
                            ((max as i32 + i) as usize+1, true)
                        }
                        else if i as usize > max {
                            (i as usize - max - 1, true)
                        } else {
                            (i as usize, false)
                        }
                    }

                    // Left
                    let (p_x, wrapped) = wrap(x_i as i32 - 1, w_max);
                    let offset = glm::Vec2::new(-self.size * wrapped as i32 as f32, 0.0);
                    other_partitions.push((&self.partitions[to_id(p_x, y_i, w)], offset)); 
                    
                    // Bottom Left
                    let (p_x, wrapped_x) = wrap(x_i as i32 - 1, w_max);
                    let (p_y, wrapped_y) = wrap(y_i as i32 + 1, w_max);
                    let offset = glm::Vec2::new(-self.size * wrapped_x as i32 as f32, self.size * wrapped_y as i32 as f32);
                    other_partitions.push((&self.partitions[to_id(p_x, p_y, w)], offset)); 

                    // Top Left
                    let (p_x, wrapped_x) = wrap(x_i as i32 - 1, w_max);
                    let (p_y, wrapped_y) = wrap(y_i as i32 - 1, w_max);
                    let offset = glm::Vec2::new(-self.size * wrapped_x as i32 as f32, -self.size * wrapped_y as i32 as f32);
                    other_partitions.push((&self.partitions[to_id(p_x, p_y, w)], offset)); 
                    
                    // Right
                    let (p_x, wrapped) = wrap(x_i as i32 + 1, w_max);
                    let offset = glm::Vec2::new(self.size * wrapped as i32 as f32, 0.0);
                    other_partitions.push((&self.partitions[to_id(p_x, y_i, w)], offset)); 
                    
                    // Bottom Right
                    let (p_x, wrapped_x) = wrap(x_i as i32 + 1, w_max);
                    let (p_y, wrapped_y) = wrap(y_i as i32 + 1, w_max);
                    let offset = glm::Vec2::new(self.size * wrapped_x as i32 as f32, self.size * wrapped_y as i32 as f32);
                    other_partitions.push((&self.partitions[to_id(p_x, p_y, w)], offset)); 
                
                    // Top Right
                    let (p_x, wrapped_x) = wrap(x_i as i32 + 1, w_max);
                    let (p_y, wrapped_y) = wrap(y_i as i32 - 1, w_max);
                    let offset = glm::Vec2::new(self.size * wrapped_x as i32 as f32, -self.size * wrapped_y as i32 as f32);
                    other_partitions.push((&self.partitions[to_id(p_x, p_y, w)], offset)); 
                    
                    // Bottom
                    let (p_y, wrapped) = wrap(y_i as i32 + 1, w_max);
                    let offset = glm::Vec2::new(0.0, self.size * wrapped as i32 as f32);
                    other_partitions.push((&self.partitions[to_id(x_i, p_y, w)], offset)); 
                    
                    // Top
                    let (p_y, wrapped) = wrap(y_i as i32 - 1, w_max);
                    let offset = glm::Vec2::new(0.0, -self.size * wrapped as i32 as f32);
                    other_partitions.push((&self.partitions[to_id(x_i, p_y, w)], offset));   
                }
            }

            for &index in &partition.particles {
                particle_velocities_mut[index] *= drag.powf(delta_time);

                for &(other_partition, offset) in &other_partitions{
                    for &other_index in &other_partition.particles {
                        let mut vec: glm::Vec2 = particle_positions_mut[other_index] - particle_positions_mut[index] + offset;
                        let mut flt: f32 = vec.x*vec.x+vec.y*vec.y;
    
                        if flt == 0.0 {
                            continue;
                        } 

                        // Reusing variables to save memory throughput
                        flt = flt.sqrt();
                        vec = vec / flt * particle_speed;
                        
                        // https://www.desmos.com/calculator/xjmwts0q8l
                        if flt > particle_settings.min_r {
                            let c = color_table.table[particle_color_ids_mut[index] as usize][particle_color_ids_mut[other_index] as usize];
                            // Old equation was removed because it was too costly: ((PI*(dist - particle_settings.min_r)) / (particle_settings.max_r - particle_settings.min_r)).sin();
                            let v = 1.0 - (1.0 + min_r_norm - 2.0 * (flt/particle_settings.max_r).min(1.0)).abs() / (1.0 - min_r_norm);
                            
                            particle_velocities_mut[index] += vec * c * v;
                        } else {
                            particle_velocities_mut[index] += vec * (flt / particle_settings.min_r - 1.0);
                        }
                    }
                }
            }
        });

        self.velocity_update_time = start.elapsed().as_secs_f32()*1000.0;

        let start = std::time::Instant::now();

        let barrier = self.half_size;

        for i in 0..self.particle_velocities.len() {
            self.particle_positions[i] += self.particle_velocities[i] * delta_time;
        }

        self.particle_positions.iter_mut().for_each(|position| {
            match world_settings.wrapping {
                ParticleWrapping::Wrap => {
                    if position.x > barrier-BARRIER_MARGIN {
                        position.x = -barrier+BARRIER_MARGIN*2.0;
                    } if position.x < -barrier+BARRIER_MARGIN {
                        position.x = barrier-0.2;
                    }

                    if position.y > barrier-BARRIER_MARGIN {
                        position.y = -barrier+BARRIER_MARGIN*2.0;
                    } if position.y < -barrier+BARRIER_MARGIN {
                        position.y = barrier-BARRIER_MARGIN*2.0;
                    }
                }
                ParticleWrapping::Barrier => {
                    position.x = position.x.clamp(-barrier+BARRIER_MARGIN, barrier-BARRIER_MARGIN);
                    position.y = position.y.clamp(-barrier+BARRIER_MARGIN, barrier-BARRIER_MARGIN);
                }
            }
        });

        self.position_update_time = start.elapsed().as_secs_f32()*1000.0;
    }

    pub fn new_partitions(&mut self, world_settings: &WorldSettings, particle_settings: &ParticleSettings) {
        let world_size = world_settings.size;
        let cell_size = particle_settings.max_r;

        // The cell size always has to be equal or greater to particle_max_r (Max influnce radius of a particle)
        let cell_count_floor = (world_size / cell_size).floor() as usize;
        let cell_size = world_size / cell_count_floor as f32;

        let cell_count = (world_size / cell_size).ceil() as usize;

        self.size = world_size;
        self.half_size = world_size / 2.0;
        self.partitions = vec![PartitionCell::new(); cell_count*cell_count];
        self.cell_count = cell_count;
        self.cell_size = cell_size;

        self.particle_positions.iter_mut().for_each(|position| {
            if position.x > self.half_size-BARRIER_MARGIN {
                position.x = -self.half_size+BARRIER_MARGIN*2.0;
            } if position.x < -self.half_size+BARRIER_MARGIN {
                position.x = self.half_size-0.2;
            }

            if position.y > self.half_size-BARRIER_MARGIN {
                position.y = -self.half_size+BARRIER_MARGIN*2.0;
            } if position.y < -self.half_size+BARRIER_MARGIN {
                position.y = self.half_size-BARRIER_MARGIN*2.0;
            }
        });
    }

    pub fn update_partitions(&mut self) {
        let start = std::time::Instant::now();

        self.partitions.iter_mut().for_each(|partition|{
            partition.particles.clear();
        });
    
        self.particle_positions.clone().iter().enumerate().for_each(|(index, position)| {
            let id = self.get_partition_id(&position);

            self.partitions[id].particles.push(index);
        });

        self.partition_update_time = start.elapsed().as_secs_f32()*1000.0;
    }

    pub fn get_closest_particle_id(&self, pos: &glm::Vec2) -> Option<usize> {
        if pos.x.abs() >= self.half_size || pos.y.abs() >= self.half_size {
           return None;
        }

        let partition = &self.partitions[self.get_partition_id(pos)];

        let mut closest_index = 0;
        let mut closest_distance = f32::INFINITY;

        for (index, position) in partition.particles.iter().map(|&index| (index, &self.particle_positions[index])) {
            let diff = position - pos;
            let sq_dist = diff.x*diff.x+diff.y*diff.y;

            if sq_dist < closest_distance {
                closest_distance = sq_dist;
                closest_index = index;
            }
        }

        Some(closest_index)
    }

    fn get_partition_id(&self, pos: &glm::Vec2) -> usize {
        let (x, y) = (
            ((pos.x + self.half_size) / self.cell_size).floor() as usize, 
            ((pos.y + self.half_size) / self.cell_size).floor() as usize
        );

        y * self.cell_count + x
    }
}