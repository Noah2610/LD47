use super::system_prelude::*;
use amethyst::ui::{UiText, UiTransform};
use deathframe::amethyst;

const OUTPUT_ID: &str = "ingame_output_label";

#[derive(Default)]
pub struct UpdateTextOutputSystem {
    cache: String,
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
        let should_update = &self.cache != &text_output.text;

        if should_update {
            self.cache = text_output.text.clone();
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
                ui_text.text = text_output.text.clone();
            }
        }
    }
}
