use crate::settings::prelude::{SceneSettings, ScenesSettings};

#[derive(Default)]
pub struct SceneManager {
    pub current_loop:           usize,
    pub should_load_next_scene: bool,
    scenes:                     Vec<SceneSettings>,
    current_scene_idx:          usize,
}

impl SceneManager {
    pub fn current_scene(&self) -> &SceneSettings {
        self.scenes
            .get(self.current_scene_idx)
            .expect("SceneManager should always have a current scene")
    }

    pub fn next_scene(&mut self) -> &SceneSettings {
        self.should_load_next_scene = false;
        let prev_idx = self.current_scene_idx;
        self.current_scene_idx =
            (self.current_scene_idx + 1) % self.scenes.len();
        if self.current_scene_idx < prev_idx {
            self.current_loop += 1;
        }
        self.current_scene()
    }
}

impl From<ScenesSettings> for SceneManager {
    fn from(settings: ScenesSettings) -> Self {
        Self {
            current_loop:           0,
            should_load_next_scene: false,
            scenes:                 settings.scenes,
            current_scene_idx:      0,
        }
    }
}
