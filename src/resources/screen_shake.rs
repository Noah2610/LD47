#[derive(Default)]
pub struct ScreenShakeRes {
    pub shake: Option<ScreenShake>,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ScreenShake {
    pub duration_ms:    u64,
    pub strength:       f32,
    pub shake_delay_ms: u64,
}
