use std::collections::HashMap;

#[derive(Default)]
pub struct TextOutput {
    pub outputs: HashMap<String, TextOutputValue>,
}

impl TextOutput {
    pub fn set(
        &mut self,
        output_name: String,
        text: String,
        does_scroll: bool,
    ) {
        let output = self.outputs.entry(output_name).or_default();
        if does_scroll {
            output.set_staged(text);
        } else {
            output.set(text);
        }
    }

    pub fn add(
        &mut self,
        output_name: String,
        text: String,
        does_scroll: bool,
    ) {
        let output = self.outputs.entry(output_name).or_default();
        if does_scroll {
            output.add_staged(text);
        } else {
            output.add(text);
        }
    }

    pub fn clear(&mut self, output_name: &str) {
        self.outputs
            .get_mut(output_name)
            .map(TextOutputValue::clear);
    }
}

#[derive(Default)]
pub struct TextOutputValue {
    pub text:   String,
    pub staged: Vec<String>,
}

impl TextOutputValue {
    pub fn set(&mut self, text: String) {
        self.staged.clear();
        self.text = text;
    }

    pub fn add(&mut self, text: String) {
        for staged in self.staged.drain(..) {
            self.text.push_str(staged.as_str());
        }
        self.text.push_str(text.as_str());
    }

    fn set_staged(&mut self, text: String) {
        self.clear();
        self.staged = text.chars().fold(Vec::new(), |mut acc, chr| {
            if chr.is_ascii_whitespace() {
                acc.last_mut().map(|last| last.push(chr));
            } else {
                acc.push(chr.to_string());
            }
            acc
        });
    }

    fn add_staged(&mut self, text: String) {
        self.staged.push(String::from("\n"));
        for chr in text.chars() {
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
}
