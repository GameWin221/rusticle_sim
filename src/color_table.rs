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
            table: Self::gen_new_table(color_count)
        }
    }

    fn gen_new_table(color_count: usize) -> Vec<Vec<f32>> {
        (0..color_count).map(|_| {
            (0..color_count).map(|_| {
                rand::thread_rng().gen_range(-1.0..=1.0)
            }).collect()
        }).collect()
    }

    pub fn new_table(&mut self) {
        self.table = Self::gen_new_table(self.colors.len());
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