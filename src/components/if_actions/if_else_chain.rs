use crate::components::prelude::{ActionType, IfCondition};

#[derive(Deserialize, Clone)]
pub struct IfElseChain {
    pub branches: Vec<IfElseChainBranch>,
    #[serde(alias = "else", default)]
    pub failure:  Option<Vec<ActionType>>,
}

#[derive(Deserialize, Clone)]
pub struct IfElseChainBranch {
    #[serde(alias = "if")]
    pub condition: IfCondition,
    #[serde(alias = "then")]
    pub success:   Vec<ActionType>,
}
