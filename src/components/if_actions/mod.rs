pub mod prelude {
    pub use super::condition::prelude::*;
    pub use super::IfAction;
    pub use super::IfActions;
}

mod condition;

use super::component_prelude::*;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct IfActions {
    actions: Vec<IfAction>,
}

#[derive(Deserialize, Clone)]
pub struct IfAction {
    #[serde(alias = "if")]
    pub condition: IfCondition,
    #[serde(alias = "then")]
    pub success:   Vec<ActionType>,
    #[serde(alias = "else", default)]
    pub failure:   Option<Vec<ActionType>>,
}

impl ActionQueue for IfActions {
    type Action = IfAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
