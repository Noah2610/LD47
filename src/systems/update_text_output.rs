use super::system_prelude::*;
use amethyst::ui::{UiText, UiTransform};
use deathframe::amethyst;
use std::collections::HashMap;

#[derive(Default)]
pub struct UpdateTextOutputSystem {
    cache: HashMap<String, String>,
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
        for (output_id, text_output) in text_output.outputs.iter() {
            let should_update = self
                .cache
                .get(output_id)
                .map(|cached| cached != &text_output.text)
                .unwrap_or(true);

            if should_update {
                self.cache
                    .insert(output_id.to_string(), text_output.text.clone());
                if let Some(ui_text) = (&ui_transform_store, &mut ui_text_store)
                    .join()
                    .find_map(|(transform, text)| {
                        if &transform.id == output_id {
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
}
