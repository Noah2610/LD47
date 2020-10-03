use super::system_prelude::*;
use crate::input::prelude::IngameAxis;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadExpect<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Movement>,
        WriteStorage<'a, BaseFriction>,
    );

    fn run(
        &mut self,
        (
            time,
            input_manager,
            player_store,
            mut velocity_store,
            mut movement_store,
            mut base_friction_store,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        if let Some(x) = input_manager.axis_value(IngameAxis::WalkX) {
            for (player, velocity, movement, friction) in (
                &player_store,
                &mut velocity_store,
                &mut movement_store,
                &mut base_friction_store,
            )
                .join()
            {
                if player.is_in_control() {
                    if x != 0.0 {
                        if velocity.x.signum() != x.signum() {
                            velocity.x = 0.0;
                        }

                        friction.set_enabled(&Axis::X, false);
                        let accel = movement.acceleration * x * dt;
                        velocity.increase_with_max(
                            &Axis::X,
                            accel,
                            movement.max_velocity,
                        );
                    } else {
                        friction.set_enabled(&Axis::X, true);
                    }
                }
            }
        }
    }
}
