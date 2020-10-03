type DurationMs = u64;
type Strength = f32;

#[derive(Default)]
pub struct ScreenShake {
    pub shake: Option<(DurationMs, Strength)>,
}
