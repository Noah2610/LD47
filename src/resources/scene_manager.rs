use crate::settings::prelude::{SceneSettings, ScenesSettings};
use deathframe::amethyst::ecs::Entity;
use std::collections::HashSet;

#[derive(Default)]
pub struct SceneManager {
    pub current_loop:           usize,
    pub should_load_next_scene: bool,
    pub triggered_init_events:  HashSet<Entity>,
    pub current_scene_idx:      usize,
    scenes:                     Vec<SceneSettings>,
}

impl SceneManager {
    pub fn current_scene(&self) -> &SceneSettings {
        self.scenes
            .get(self.current_scene_idx)
            .expect("SceneManager should always have a current scene")
    }

    pub fn next_scene(&mut self) -> &SceneSettings {
        self.should_load_next_scene = false;
        self.triggered_init_events = HashSet::new();
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
            triggered_init_events:  HashSet::new(),
            current_scene_idx:      0,
            scenes:                 settings.scenes,
        }
    }
}
