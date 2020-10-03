use deathframe::physics::CollisionTag as CTag;

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(from = "String")]
pub struct CollisionTag(pub String);

impl From<String> for CollisionTag {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl CTag for CollisionTag {
    fn collides_with(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub type SolidTag = CollisionTag;
