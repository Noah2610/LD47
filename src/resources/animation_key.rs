#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Debug)]
pub enum AnimationKey {
    Idle,
    Walk,
    Custom(String),
}

impl AnimationKey {
    pub fn is_custom(&self) -> bool {
        match self {
            AnimationKey::Custom(_) => true,
            _ => false,
        }
    }
}
