use super::system_prelude::*;
use amethyst::ui::{UiText, UiTransform};
use deathframe::amethyst;

const OUTPUT_ID: &str = "ingame_output_label";

pub struct UpdateTextOutputSystem {
    cache: Vec<String>,
}

impl<'a> System<'a> for UpdateTextOutputSystem {
    type SystemData = (
        Write<'a, TextOutput>,
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (
            mut text_output,
            ui_transform_store,
            mut ui_text_store,
        ): Self::SystemData,
    ) {
        if &text_output.lines != &self.cache {
            self.cache = text_output.lines.clone();

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
                ui_text.text = text_output.lines.join("\n");
            }
        }
    }
}

impl Default for UpdateTextOutputSystem {
    fn default() -> Self {
        Self { cache: Vec::new() }
    }
}
