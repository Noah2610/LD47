use super::component_prelude::*;
use rand::Rng;
use std::collections::HashMap;

#[derive(Component, Deserialize, Clone)]
#[storage(DenseVecStorage)]
#[serde(from = "HashMap<String, TextLinesGroup>", deny_unknown_fields)]
pub struct TextLines {
    groups: HashMap<String, TextLinesGroup>,
}

impl TextLines {
    pub fn next_line(&mut self, group_name: &str) -> Option<&str> {
        self.groups
            .get_mut(group_name)
            .map(TextLinesGroup::next_line)
    }
}

impl From<HashMap<String, TextLinesGroup>> for TextLines {
    fn from(groups: HashMap<String, TextLinesGroup>) -> Self {
        Self { groups }
    }
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TextLinesGroup {
    pub lines:    Vec<String>,
    pub behavior: TextLinesBehavior,
}

impl TextLinesGroup {
    fn next_line(&mut self) -> &str {
        match &mut self.behavior {
            TextLinesBehavior::Random => {
                let mut rng = rand::thread_rng();
                let idx: usize = rng.gen_range(0, self.lines.len());
                self.lines[idx].as_str()
            }
            TextLinesBehavior::Sequence {
                does_loop,
                line_idx,
            } => {
                let cur_idx = *line_idx;
                let next_line_idx = cur_idx + 1;
                if next_line_idx < self.lines.len() {
                    *line_idx = next_line_idx;
                } else {
                    if *does_loop {
                        *line_idx = 0;
                    } else {
                        // DO NOTHING
                        // stay at last line idx
                    }
                }
                self.lines[cur_idx].as_str()
            }
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub enum TextLinesBehavior {
    Random,
    Sequence {
        #[serde(alias = "loop")]
        does_loop: bool,
        #[serde(skip)]
        line_idx:  usize,
    },
}
