use super::system_prelude::*;
use climer::Timer;
use std::time::Duration;

const PRINT_CHAR_DELAY_MS: u64 = 10;

pub struct HandleTextOutputScrollingSystem {
    timer: Timer,
}

impl<'a> System<'a> for HandleTextOutputScrollingSystem {
    type SystemData = Write<'a, TextOutput>;

    fn run(&mut self, mut text_output: Self::SystemData) {
        if !text_output.staged.is_empty() {
            if !self.timer.state.is_running() {
                self.timer.start().unwrap();
            }
            self.timer.update().unwrap();
            if self.timer.state.is_finished() {
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

impl Default for HandleTextOutputScrollingSystem {
    fn default() -> Self {
        Self {
            timer: Timer::new(
                Some(Duration::from_millis(PRINT_CHAR_DELAY_MS).into()),
                None,
            ),
        }
    }
}
