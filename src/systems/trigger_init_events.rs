use super::system_prelude::*;

#[derive(Default)]
pub struct TriggerInitEventsSystem;

impl<'a> System<'a> for TriggerInitEventsSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, SceneManager>,
        WriteStorage<'a, EventsRegister>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut scene_manager,
            mut events_register_store,
        ): Self::SystemData,
    ) {
        for (entity, events_register) in
            (&entities, &mut events_register_store).join()
        {
            if !scene_manager.triggered_init_events.contains(&entity) {
                scene_manager.triggered_init_events.insert(entity);
                events_register.trigger_event(&EventType::Init);
            }
        }
    }
}
