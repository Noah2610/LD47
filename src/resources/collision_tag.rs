use deathframe::physics::CollisionTag as CTag;

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum CollisionTag {
    Player,
    Solid,
    Interactable,
    WithPlayer,
}

impl CTag for CollisionTag {
    fn collides_with(&self, other: &Self) -> bool {
        use self::CollisionTag as Tag;

        match (self, other) {
            (Tag::Player, Tag::Solid) => true,
            (Tag::Player, Tag::Interactable) => true,
            (Tag::WithPlayer, Tag::Player) => true,
            _ => false,
        }
    }
}

pub type SolidTag = CollisionTag;
