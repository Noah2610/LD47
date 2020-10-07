pub use crate::components::prelude::{ActionType, IfCondition};

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct If {
    #[serde(alias = "if")]
    pub condition: IfCondition,
    #[serde(alias = "then")]
    pub success:   Vec<ActionType>,
    #[serde(alias = "else", default)]
    pub failure:   Option<Vec<ActionType>>,
}
