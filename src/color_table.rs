use rand::Rng;
pub struct ColorTable {
    pub colors: Vec<glm::Vec3>,
    pub table: Vec<Vec<f32>>,
}

impl ColorTable {
    pub fn new(color_palette: Vec<glm::Vec3>) -> Self {
        let color_count = color_palette.len();

        Self {
            colors: color_palette,
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
}