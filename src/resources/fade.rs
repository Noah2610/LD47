#[derive(Default)]
pub struct FadeRes {
    pub fade: Option<Fade>,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Fade {
    #[serde(alias = "type")]
    pub fade_type:   FadeType,
    pub duration_ms: u64,
    #[serde(default = "default_color")]
    pub color:       [f32; 3],
}

#[derive(Deserialize, Clone)]
pub enum FadeType {
    FadeIn,
    FadeOut,
}

const fn default_color() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}
