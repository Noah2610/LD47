#[derive(Default)]
pub struct TextOutput {
    pub lines: Vec<String>,
}

impl TextOutput {
    pub fn set(&mut self, lines: Vec<String>) {
        self.lines = lines;
        self.truncate();
    }

    pub fn add(&mut self, mut lines: Vec<String>) {
        self.lines.append(&mut lines);
        self.truncate();
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.truncate();
    }

    fn truncate(&mut self) {
        const MAX: usize = 5;
        let len = self.lines.len();
        if len > MAX {
            self.lines = self.lines[len - MAX ..].to_vec();
        }
    }
}
