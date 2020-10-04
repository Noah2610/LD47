use std::vec::IntoIter;

#[derive(Default)]
pub struct TextOutput {
    pub text:   String,
    pub staged: Vec<String>,
}

impl TextOutput {
    pub fn set(&mut self, lines: Vec<String>) {
        self.clear();
        self.staged =
            lines.join("\n").chars().fold(Vec::new(), |mut acc, chr| {
                if chr.is_ascii_whitespace() {
                    acc.last_mut().map(|last| last.push(chr));
                } else {
                    acc.push(chr.to_string());
                }
                acc
            });
    }

    pub fn add(&mut self, lines: Vec<String>) {
        self.staged.push(String::from("\n"));
        for chr in lines.join("\n").chars() {
            if chr.is_ascii_whitespace() {
                self.staged.last_mut().map(|last| last.push(chr));
            } else {
                self.staged.push(chr.to_string());
            }
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.staged.clear();
    }

    // TODO
    // fn truncate(&mut self) {
    //     const MAX: usize = 5;
    //     let len = self.lines.len();
    //     if len > MAX {
    //         self.lines = self.lines[len - MAX ..].to_vec();
    //     }
    // }
}
