pub struct Model {
    pub frame: u64,
}

impl Model {
    pub fn default() -> Self {
        Self {
            frame: 0,
        }
    }

    pub fn tick(&mut self) {
        self.frame += 1;

    }
}