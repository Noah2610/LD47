use super::system_prelude::*;
use amethyst::ui::{UiText, UiTransform};
use climer::Timer;
use deathframe::amethyst;
use std::time::Duration;

const CHAR_PRINT_DELAY_MS: u64 = 50;
const OUTPUT_ID: &str = "ingame_output_label";

pub struct UpdateTextOutputSystem {
    cache:             Vec<String>,
    to_print:          Vec<String>,
    chars_print_timer: Option<Timer>,
}

impl<'a> System<'a> for UpdateTextOutputSystem {
    type SystemData = (
        Read<'a, TextOutput>,
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (text_output, ui_transform_store, mut ui_text_store): Self::SystemData,
    ) {
        let mut should_update = false;

        if &text_output.lines != &self.cache {
            self.cache = text_output.lines.clone();
            self.to_print = self.cache.clone();
            self.to_print
                .last_mut()
                .map(|last_line| *last_line = String::new());
        }

        if &self.to_print != &self.cache {
            let should_add_char = {
                let timer = self.chars_print_timer.get_or_insert({
                    let mut timer = Timer::new(
                        Some(Duration::from_millis(CHAR_PRINT_DELAY_MS).into()),
                        None,
                    );
                    timer.start().unwrap();
                    timer
                });

                timer.update().unwrap();
                if timer.state.is_finished() {
                    timer.start().unwrap();
                    true
                } else {
                    false
                }
            };
            if should_add_char {
                let last_cache_line = self.cache.last().cloned();
                self.to_print
                    .last_mut()
                    .and_then(|to_print| {
                        last_cache_line.map(|last| (to_print, last))
                    })
                    .and_then(|(to_print, last_line)| {
                        let len = to_print.len();
                        last_line
                            .get(len .. len + 1)
                            .map(|c| (to_print, c.to_string()))
                    })
                    .map(|(to_print, new)| to_print.push_str(new.as_str()));
            }

            should_update = true;
        }

        if should_update {
            if let Some(ui_text) = (&ui_transform_store, &mut ui_text_store)
                .join()
                .find_map(|(transform, text)| {
                    if transform.id.as_str() == OUTPUT_ID {
                        Some(text)
                    } else {
                        None
                    }
                })
            {
                ui_text.text = self.to_print.join("\n");
            }
        }
    }
}

impl Default for UpdateTextOutputSystem {
    fn default() -> Self {
        Self {
            cache:             Vec::new(),
            to_print:          Vec::new(),
            chars_print_timer: None,
        }
    }
}
