use super::system_prelude::*;
use climer::Timer;
use deathframe::amethyst::utils::ortho_camera::{
    CameraOrtho,
    CameraOrthoWorldCoordinates,
};
use rand::prelude::Rng;
use std::time::Duration;

const SHAKE_DELAY_MS: u64 = 10;

#[derive(Default)]
pub struct HandleScreenShakeSystem {
    shaking: Option<ShakingState>,
}

impl<'a> System<'a> for HandleScreenShakeSystem {
    type SystemData = (
        Write<'a, ScreenShake>,
        WriteStorage<'a, AmethystCamera>,
        WriteStorage<'a, CameraOrtho>,
    );

    fn run(
        &mut self,
        (
            mut screen_shake,
            mut camera_store,
            mut camera_ortho_store,
        ): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();

        for (camera, camera_ortho) in
            (&mut camera_store, &mut camera_ortho_store).join().next()
        {
            if let Some(shake) = screen_shake.shake.take() {
                self.shaking = Some(ShakingState {
                    timer:         {
                        let mut timer = Timer::new(
                            Some(Duration::from_millis(shake.0).into()),
                            None,
                        );
                        timer.start().unwrap();
                        timer
                    },
                    shake_timer:   {
                        let mut timer = Timer::new(
                            Some(Duration::from_millis(SHAKE_DELAY_MS).into()),
                            None,
                        );
                        timer.start().unwrap();
                        timer
                    },
                    strength:      shake.1,
                    camera_coords: camera_ortho.world_coordinates.clone(),
                });
            }

            if let Some(mut shaking) = self.shaking.take() {
                shaking.timer.update().unwrap();
                if shaking.timer.state.is_finished() {
                    camera_ortho.world_coordinates = shaking.camera_coords;
                    *camera = AmethystCamera::orthographic(
                        camera_ortho.world_coordinates.left,
                        camera_ortho.world_coordinates.right,
                        camera_ortho.world_coordinates.bottom,
                        camera_ortho.world_coordinates.top,
                        camera_ortho.world_coordinates.near,
                        camera_ortho.world_coordinates.far,
                    );
                } else {
                    shaking.shake_timer.update().unwrap();
                    if shaking.shake_timer.state.is_finished() {
                        shaking.shake_timer.start().unwrap();
                        let offset: (f32, f32) = (
                            rng.gen_range(-shaking.strength, shaking.strength),
                            rng.gen_range(-shaking.strength, shaking.strength),
                        );
                        camera_ortho.world_coordinates =
                            CameraOrthoWorldCoordinates {
                                top:    shaking.camera_coords.top + offset.1,
                                bottom: shaking.camera_coords.bottom + offset.1,
                                left:   shaking.camera_coords.left + offset.0,
                                right:  shaking.camera_coords.right + offset.0,
                                near:   shaking.camera_coords.near,
                                far:    shaking.camera_coords.far,
                            };
                        *camera = AmethystCamera::orthographic(
                            camera_ortho.world_coordinates.left,
                            camera_ortho.world_coordinates.right,
                            camera_ortho.world_coordinates.bottom,
                            camera_ortho.world_coordinates.top,
                            camera_ortho.world_coordinates.near,
                            camera_ortho.world_coordinates.far,
                        );
                    }
                    self.shaking = Some(shaking);
                }
            }
        }
    }
}

struct ShakingState {
    pub timer:         Timer,
    pub shake_timer:   Timer,
    pub strength:      f32,
    pub camera_coords: CameraOrthoWorldCoordinates,
}
