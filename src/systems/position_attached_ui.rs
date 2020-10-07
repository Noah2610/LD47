use super::system_prelude::*;
use amethyst::core::math::{Point3, Vector2};
use amethyst::renderer::Camera;
use amethyst::ui::UiTransform;
use amethyst::window::DisplayConfig;
use deathframe::amethyst;

#[derive(Default)]
pub struct PositionAttachedUiSystem;

impl<'a> System<'a> for PositionAttachedUiSystem {
    type SystemData = (
        ReadExpect<'a, DisplayConfig>,
        ReadStorage<'a, AttachUi>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, UiTransform>,
        ReadStorage<'a, Camera>,
    );

    fn run(
        &mut self,
        (
            display_config,
            attach_ui_store,
            transform_store,
            mut ui_transform_store,
            camera_store,
        ): Self::SystemData,
    ) {
        if let Some((camera, camera_transform)) =
            (&camera_store, &transform_store).join().next()
        {
            let screen_dim = {
                let dim = display_config.dimensions.as_ref().unwrap();
                Vector2::new(dim.0 as f32, dim.1 as f32)
            };

            for (attach_ui, transform) in
                (&attach_ui_store, &transform_store).join()
            {
                let screen_pos = {
                    let trans = transform.translation();
                    let mut screen_pos = camera.world_to_screen(
                        Point3::new(
                            trans.x + attach_ui.offset.0,
                            trans.y + attach_ui.offset.1,
                            trans.z,
                        ),
                        screen_dim,
                        camera_transform,
                    );
                    screen_pos.x -= screen_dim.x * 0.5;
                    screen_pos.y = screen_dim.y * 0.5 - screen_pos.y;
                    screen_pos
                };

                for ui_transform in (&mut ui_transform_store).join() {
                    if &ui_transform.id == &attach_ui.target_id {
                        ui_transform.local_x = screen_pos.x;
                        ui_transform.local_y = screen_pos.y;
                    }
                }
            }
        }
    }
}
