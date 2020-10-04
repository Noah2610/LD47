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
}

#[derive(Deserialize, Clone)]
pub enum FadeType {
    FadeIn,
    FadeOut,
}
