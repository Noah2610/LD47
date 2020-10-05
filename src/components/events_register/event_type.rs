use crate::input::prelude::IngameAction;
use crate::resources::CollisionTag;
use deathframe::physics::query::prelude::QueryExpression;

#[derive(Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    OnInteract,
    OnCollision(QueryExpression<CollisionTag>),
    OnTimerFinish(String),
    Init,
    OnKeyDown(IngameAction),
    OnKeyUp(IngameAction),
    OnKeyPressed(IngameAction),
}
