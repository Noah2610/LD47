use crate::components::prelude::*;
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

    const CAMERA_Z: f32 = 10.0;

    let size = level_size;

    let camera = Camera::standard_2d(size.w, size.h);
    let mut camera_ortho =
        CameraOrtho::normalized(CameraNormalizeMode::Contain);
    let half_size = size.half();
    camera_ortho.world_coordinates = CameraOrthoWorldCoordinates {
        top:    half_size.h,
        bottom: -half_size.h,
        left:   -half_size.w,
        right:  half_size.w,
        // TODO
        near:   -10.0,
        far:    10.0,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(half_size.w, half_size.h, CAMERA_Z);

    world
        .create_entity()
        .with(transform)
        .with(size)
        .with(camera)
        .build();

    Ok(())
}
