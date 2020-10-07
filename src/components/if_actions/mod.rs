pub mod prelude {
    pub use super::condition::prelude::*;
    pub use super::if_else_chain::IfElseChain;
    pub use super::r#if::If;
    pub use super::IfAction;
    pub use super::IfActions;
}

mod condition;
mod r#if;
mod if_else_chain;

use super::component_prelude::*;
use deathframe::amethyst::ecs::Entity;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct IfActions {
    actions: Vec<IfAction>,
}

impl ActionQueue for IfActions {
    type Action = IfAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields, untagged)]
pub enum IfAction {
    If(If),
    IfElseChain(IfElseChain),
}

impl IfAction {
    pub fn run(
        self,
        entity: Entity,
        stores: &IfStorages,
    ) -> Option<Vec<ActionType>> {
        match self {
            Self::If(if_action) => {
                if if_action.condition.passes(entity, &stores) {
                    Some(if_action.success)
                } else {
                    if let Some(failure) = if_action.failure {
                        Some(failure)
                    } else {
                        None
                    }
                }
            }
            Self::IfElseChain(chain) => chain
                .branches
                .into_iter()
                .find_map(|branch| {
                    if branch.condition.passes(entity, &stores) {
                        Some(branch.success)
                    } else {
                        None
                    }
                })
                .or(chain.failure),
        }
    }
}
