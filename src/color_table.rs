use rand::Rng;
pub struct ColorTable {
    pub colors: Vec<glm::Vec3>,
    pub table: Vec<Vec<f32>>,
}

impl ColorTable {
    pub fn new(color_palette: &Vec<glm::Vec3>) -> Self {
        let color_count = color_palette.len();

        Self {
            colors: color_palette.clone(),
            table: Self::gen_random_table(color_count)
        }
    }

    fn gen_random_table(color_count: usize) -> Vec<Vec<f32>> {
        (0..color_count).map(|_| {
            (0..color_count).map(|_| {
                rand::thread_rng().gen_range(-1.0..=1.0)
            }).collect()
        }).collect()
    }

    fn gen_filled_table(color_count: usize, fill: f32) -> Vec<Vec<f32>> {
        (0..color_count).map(|_| {
            (0..color_count).map(|_| {
                fill
            }).collect()
        }).collect()
    }

    pub fn new_random_table(&mut self) {
        self.table = Self::gen_random_table(self.colors.len());
    }
    pub fn new_filled_table(&mut self, fill: f32) {
        self.table = Self::gen_filled_table(self.colors.len(), fill);
    }

    pub fn add_color(&mut self) {
        let color = glm::Vec3::new(
            rand::thread_rng().gen_range(0.0..=1.0),
            rand::thread_rng().gen_range(0.0..=1.0),
            rand::thread_rng().gen_range(0.0..=1.0),
        );

        self.colors.push(color);

        self.table.iter_mut().for_each(|row| {
            row.push(0.0);
        });

        self.table.push(vec![0.0; self.colors.len()]);
    }
    pub fn remove_color(&mut self) {
        if self.colors.len() <= 1 {
            return;
        }

        self.colors.pop();

        self.table.iter_mut().for_each(|row| {
            row.pop();
        });

        self.table.pop();
    }
}