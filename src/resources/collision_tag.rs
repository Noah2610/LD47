use deathframe::physics::CollisionTag as CTag;

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum CollisionTag {
    Player,
    Solid,
    Interactable,
    Boss,
    BossTableHitbox,
    /// Boss collides with this entity.
    BossCollidesWithSelf,
    /// This entity collides with boss.
    SelfCollidesWithBoss,
    /// Player collides with this entity.
    PlayerCollidesWithSelf,
    /// This entity collides with player.
    SelfCollidesWithPlayer,
}

impl CTag for CollisionTag {
    fn collides_with(&self, other: &Self) -> bool {
        use self::CollisionTag as Tag;

        match (self, other) {
            (Tag::Player, Tag::Solid) => true,
            (Tag::Player, Tag::Interactable) => true,
            (Tag::Player, Tag::Boss) => true,
            (Tag::Player, Tag::PlayerCollidesWithSelf) => true,
            (Tag::SelfCollidesWithPlayer, Tag::Player) => true,
            (Tag::Boss, Tag::Player) => true,
            (Tag::Boss, Tag::BossTableHitbox) => true,
            (Tag::Boss, Tag::BossCollidesWithSelf) => true,
            (Tag::SelfCollidesWithBoss, Tag::Boss) => true,
            _ => false,
        }
    }
}

pub type SolidTag = CollisionTag;
