use crate::components::prelude::*;
use crate::settings::prelude::CameraSettings;
use amethyst::ecs::{Builder, World, WorldExt};
use deathframe::amethyst;

pub(super) fn build_camera(
    world: &mut World,
    level_size: Size,
) -> amethyst::Result<()> {
    use amethyst::renderer::Camera;
    use amethyst::utils::ortho_camera::{
        CameraNormalizeMode,
        CameraOrtho,
        CameraOrthoWorldCoordinates,
    };

    let settings = (*world.read_resource::<CameraSettings>()).clone();

    let size = settings.size;

    let camera = Camera::standard_2d(size.w, size.h);
    let mut camera_ortho =
        CameraOrtho::normalized(CameraNormalizeMode::Contain);
    let half_size = size.half();
    camera_ortho.world_coordinates = CameraOrthoWorldCoordinates {
        top:    half_size.h,
        bottom: -half_size.h,
        left:   -half_size.w,
        right:  half_size.w,
        near:   0.0,
        far:    20.0,
    };

    let level_center = level_size.half();
    let mut transform = Transform::default();
    transform.set_translation_xyz(level_center.w, level_center.h, settings.z);

    world
        .create_entity()
        .with(Loader::new(size.w, size.h))
        .with(transform)
        .with(size)
        .with(camera)
        .with(camera_ortho)
        .build();

    Ok(())
}
