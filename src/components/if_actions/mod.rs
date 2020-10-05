pub mod prelude {
    pub use super::condition::prelude::*;
    pub use super::IfActions;
}

mod condition;

use super::component_prelude::*;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct IfActions {}
