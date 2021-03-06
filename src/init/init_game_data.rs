use crate::resources::*;
use crate::settings::Settings;
use crate::states::aliases::{CustomData, GameDataBuilder};
use deathframe::amethyst;

pub(super) fn build_game_data<'a, 'b>(
    settings: &Settings,
) -> amethyst::Result<GameDataBuilder<'a, 'b>> {
    use crate::input::prelude::*;
    use crate::systems::prelude::*;
    use amethyst::core::transform::TransformBundle;
    use amethyst::renderer::types::DefaultBackend;
    use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle};
    use amethyst::ui::{RenderUi, UiBundle};
    use amethyst::utils::fps_counter::FpsCounterBundle;
    use amethyst::utils::ortho_camera::CameraOrthoSystem;
    use deathframe::bundles::*;

    let transform_bundle = TransformBundle::new();
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config(settings.display_config.clone())
                .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderUi::default())
        .with_plugin(RenderFlat2D::default());
    let audio_bundle = AudioBundle::<SoundKey, SongKey>::default();
    let menu_input_bundle = MenuBindings::bundle()?;
    let ingame_input_bundle = IngameBindings::bundle()?;
    let physics_bundle =
        PhysicsBundle::<CollisionTag, SolidTag>::new().with_deps(&[]);
    let animation_bundle = AnimationBundle::<AnimationKey>::new();

    let custom_game_data = GameDataBuilder::default()
        .custom(CustomData::default())
        .dispatcher(DispatcherId::MainMenu)?
        .dispatcher(DispatcherId::Ingame)?
        .dispatcher(DispatcherId::Paused)?
        .with_core_bundle(FpsCounterBundle)?
        .with_core_bundle(transform_bundle)?
        .with_core_bundle(rendering_bundle)?
        .with_core_bundle(audio_bundle)?
        .with_core_bundle(menu_input_bundle)?
        .with_core_bundle(UiBundle::<MenuBindings>::new())?
        .with_core(PrintFpsSystem::default(), "print_fps_system", &[])?
        .with_core(CameraOrthoSystem::default(), "camera_ortho_system", &[])?
        .with_core(ScaleSpritesSystem::default(), "scale_sprites_system", &[])?
        .with_bundle(DispatcherId::Ingame, ingame_input_bundle)?
        .with_bundle(DispatcherId::Ingame, physics_bundle)?
        .with_bundle(DispatcherId::Ingame, animation_bundle)?
        .with(
            DispatcherId::MainMenu,
            InputManagerSystem::<MenuBindings>::default(),
            "main_menu_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            InputManagerSystem::<IngameBindings>::default(),
            "ingame_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Paused,
            InputManagerSystem::<MenuBindings>::default(),
            "paused_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            FollowSystem::default(),
            "follow_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            ConfineEntitiesSystem::default(),
            "confine_entities_system",
            &["move_entities_system"],
        )?
        .with(
            DispatcherId::Ingame,
            ControlPlayerSystem::default(),
            "control_player_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateAnimationsSystem::default(),
            "update_animations_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleTextOutputScrollingSystem::new(
                settings.general.text_scroll_delay_ms,
            ),
            "handle_text_output_scrolling_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateTextOutputSystem::default(),
            "update_text_output_system",
            &["handle_text_output_scrolling_system"],
        )?
        .with(
            DispatcherId::Ingame,
            HandleScreenShakeSystem::default(),
            "handle_screen_shake_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleFadeSystem::default(),
            "handle_fade_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            PositionAttachedUiSystem::default(),
            "position_attached_ui_system",
            &["move_entities_system"],
        )?
        .with(
            DispatcherId::Ingame,
            TriggerInteractionEventsSystem::default(),
            "trigger_interaction_events_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            TriggerCollisionEventsSystem::default(),
            "trigger_collision_events_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            TriggerTimerEventsSystem::default(),
            "trigger_timer_events_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            TriggerInitEventsSystem::default(),
            "trigger_init_events_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            TriggerOnKeyEventsSystem::default(),
            "trigger_on_key_events_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleEventsActionsSystem::default(),
            "handle_events_actions_system",
            &[
                "trigger_interaction_events_system",
                "trigger_collision_events_system",
                "trigger_timer_events_system",
                "trigger_init_events_system",
                "trigger_on_key_events_system",
            ],
        )?
        .with(
            DispatcherId::Ingame,
            HandleIfActionsSystem::default(),
            "handle_if_actions_system",
            &["handle_events_actions_system"],
        )?;

    Ok(custom_game_data)
}
