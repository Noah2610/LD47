pub mod prelude {
    pub use super::variable_value::VariableValue;
    pub use super::VariablesRegister;
}

mod variable_value;

use super::component_prelude::*;
use std::collections::HashMap;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct VariablesRegister {
    pub variables: HashMap<String, VariableValue>,
}
