use crate::resources::{AnimationKey, Fade, ScreenShake, SongKey, SoundKey};
use crate::settings::objects_settings::ObjectType;

#[derive(Deserialize, Clone)]
pub enum ActionType {
    Echo(String),
    ForeignObjectAction(ObjectType, Vec<ActionType>),
    SetControllable(bool),
    Show,
    Hide,
    PlayAnimation(AnimationKey),
    PushAnimation(AnimationKey),
    PopAnimation,
    FaceTowardsObject(ObjectType),
    StartTimer(String, u64),
    StopTimer(String),
    SetOutput(Vec<String>),
    AddOutput(Vec<String>),
    ClearOutput,
    ScreenShake(ScreenShake),
    NextScene,
    Fade(Fade),
    PlaySound(SoundKey),
    PlaySong(SongKey),
    PrintNextLine(String),
}
