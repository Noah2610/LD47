use super::system_prelude::*;
use climer::Timer;
use std::time::Duration;

pub struct HandleTextOutputScrollingSystem {
    timer: Timer,
}

impl<'a> System<'a> for HandleTextOutputScrollingSystem {
    type SystemData = (Write<'a, TextOutput>, Write<'a, SoundPlayer<SoundKey>>);

    fn run(&mut self, (mut text_output, mut sound_player): Self::SystemData) {
        for text_output in text_output.outputs.values_mut() {
            if !text_output.staged.is_empty() {
                if !self.timer.state.is_running() {
                    self.timer.start().unwrap();
                }
                self.timer.update().unwrap();
                if self.timer.state.is_finished() {
                    sound_player.add_action(SoundAction::Play(
                        "TextScroll".to_string(),
                    ));
                    let new = text_output.staged.remove(0);
                    text_output.text.push_str(new.as_str());
                }
            } else {
                if !self.timer.state.is_stopped() {
                    self.timer.stop().unwrap();
                }
            }
        }
    }
}

impl HandleTextOutputScrollingSystem {
    pub fn new(scroll_delay_ms: u64) -> Self {
        Self {
            timer: Timer::new(
                Some(Duration::from_millis(scroll_delay_ms).into()),
                None,
            ),
        }
    }
}
