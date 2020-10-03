use super::system_prelude::*;

const WALK_ANIM_PADDING: f32 = 1.0;

#[derive(Default)]
pub struct UpdateAnimationsSystem;

impl<'a> System<'a> for UpdateAnimationsSystem {
    type SystemData = (
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (
            mut animations_store,
            velocity_store,
            mut transform_store,
        ): Self::SystemData,
    ) {
        for (animations, velocity_opt, transform) in (
            &mut animations_store,
            velocity_store.maybe(),
            &mut transform_store,
        )
            .join()
        {
            if animations
                .current()
                .map(|anim| !anim.is_custom())
                .unwrap_or(true)
            {
                let mut target_anim = None;

                // WALK
                if let Some(velocity) = velocity_opt {
                    // FLIP SPRITE
                    if velocity.x > WALK_ANIM_PADDING {
                        let scale = transform.scale_mut();
                        scale.x = scale.x.abs() * 1.0;
                    } else if velocity.x < -WALK_ANIM_PADDING {
                        let scale = transform.scale_mut();
                        scale.x = scale.x.abs() * -1.0;
                    }

                    if (velocity.x > WALK_ANIM_PADDING
                        || velocity.x < -WALK_ANIM_PADDING)
                        && animations.has_animation(&AnimationKey::Walk)
                    {
                        target_anim = Some(AnimationKey::Walk);
                    }
                }

                // IDLE
                if target_anim.is_none()
                    && animations.has_animation(&AnimationKey::Idle)
                {
                    target_anim = Some(AnimationKey::Idle);
                }

                if let Some(anim) = target_anim {
                    let _ = animations.play(anim);
                }
            }
        }
    }
}
