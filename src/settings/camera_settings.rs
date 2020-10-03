// resources/settings/camera.ron

use crate::components::prelude::Size;

#[derive(Deserialize, Clone)]
pub struct CameraSettings {
    pub z:    f32,
    pub size: Size,
}
