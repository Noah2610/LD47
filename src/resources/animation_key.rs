#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Debug)]
pub enum AnimationKey {
    Idle,
    Walk,
    Custom(String),
}
