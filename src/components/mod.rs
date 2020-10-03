pub mod prelude {
    pub use deathframe::components::prelude::*;

    pub use super::player::Player;
}

mod component_prelude {
    pub use super::prelude::*;
    pub use deathframe::components::component_prelude::*;
}

mod player;
