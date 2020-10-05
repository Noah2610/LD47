use super::system_prelude::*;
use climer::Timer;
use std::collections::HashMap;
use std::time::Duration;

pub struct HandleTextOutputScrollingSystem {
    timers:          HashMap<String, Timer>,
    scroll_delay_ms: u64,
}

impl<'a> System<'a> for HandleTextOutputScrollingSystem {
    type SystemData = (Write<'a, TextOutput>, Write<'a, SoundPlayer<SoundKey>>);

    fn run(&mut self, (mut text_output, mut sound_player): Self::SystemData) {
        for (output_id, text_output) in text_output.outputs.iter_mut() {
            let scroll_delay_ms = self.scroll_delay_ms;
            let timer =
                self.timers.entry(output_id.to_string()).or_insert_with(|| {
                    Timer::new(
                        Some(Duration::from_millis(scroll_delay_ms).into()),
                        None,
                    )
                });
            if !text_output.staged.is_empty() {
                if !timer.state.is_running() {
                    timer.start().unwrap();
                }
                timer.update().unwrap();
                if timer.state.is_finished() {
                    sound_player.add_action(SoundAction::Play(
                        "TextScroll".to_string(),
                    ));
                    let new = text_output.staged.remove(0);
                    text_output.text.push_str(new.as_str());
                }
            } else {
                if !timer.state.is_stopped() {
                    timer.stop().unwrap();
                }
            }
        }
    }
}

impl HandleTextOutputScrollingSystem {
    pub fn new(scroll_delay_ms: u64) -> Self {
        Self {
            timers: HashMap::new(),
            scroll_delay_ms,
        }
    }
}
