use super::system_prelude::*;
use climer::Timer;
use deathframe::amethyst::ui::{UiImage, UiTransform};
use std::time::Duration;

const OVERLAY_ID: &str = "ingame_fade_overlay";
const FADE_DELAY_MS: u64 = 50;

#[derive(Default)]
pub struct HandleFadeSystem {
    fading: Option<FadingState>,
}

impl<'a> System<'a> for HandleFadeSystem {
    type SystemData = (
        Write<'a, FadeRes>,
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiImage>,
    );

    fn run(
        &mut self,
        (mut fade_res, ui_transform_store, mut ui_image_store): Self::SystemData,
    ) {
        for (transform, image) in
            (&ui_transform_store, &mut ui_image_store).join()
        {
            if &transform.id == OVERLAY_ID {
                if let UiImage::SolidColor(color) = image {
                    if let Some(fade) = fade_res.fade.take() {
                        let fade_step =
                            (FADE_DELAY_MS as f32 / fade.duration_ms as f32);
                        match &fade.fade_type {
                            FadeType::FadeIn => {
                                color[3] = 1.0;
                            }
                            FadeType::FadeOut => {
                                color[3] = 0.0;
                            }
                        }
                        self.fading = Some(FadingState {
                            fade_type: fade.fade_type,
                            fade_timer: {
                                let mut timer = Timer::new(
                                    Some(
                                        Duration::from_millis(FADE_DELAY_MS)
                                            .into(),
                                    ),
                                    None,
                                );
                                timer.start().unwrap();
                                timer
                            },
                            fade_step,
                        });
                    }

                    if let Some(mut fading) = self.fading.take() {
                        let mut should_insert_fading = true;

                        fading.fade_timer.update().unwrap();
                        if fading.fade_timer.state.is_finished() {
                            match fading.fade_type {
                                FadeType::FadeIn => {
                                    if color[3] > fading.fade_step {
                                        color[3] -= fading.fade_step;
                                        fading.fade_timer.start().unwrap();
                                    } else {
                                        should_insert_fading = false;
                                        color[3] = 0.0;
                                    }
                                }
                                FadeType::FadeOut => {
                                    if color[3] < 1.0 - fading.fade_step {
                                        color[3] += fading.fade_step;
                                        fading.fade_timer.start().unwrap();
                                    } else {
                                        should_insert_fading = false;
                                        color[3] = 1.0;
                                    }
                                }
                            }
                        }

                        if should_insert_fading {
                            self.fading = Some(fading);
                        }
                    }
                }
            }
        }
    }
}

struct FadingState {
    fade_type:  FadeType,
    fade_timer: Timer,
    fade_step:  f32,
}
