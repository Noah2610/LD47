use crate::components::prelude::{IfAction, VariableValue};
use crate::resources::{AnimationKey, Fade, ScreenShake, SongKey, SoundKey};
use crate::settings::objects_settings::ObjectType;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
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
    SetOutput {
        text:        String,
        #[serde(alias = "target", default = "default_output_target")]
        target_id:   String,
        #[serde(alias = "scroll", default)]
        does_scroll: bool,
    },
    AddOutput {
        text:        String,
        #[serde(alias = "target", default = "default_output_target")]
        target_id:   String,
        #[serde(alias = "scroll", default)]
        does_scroll: bool,
    },
    ClearOutput {
        #[serde(alias = "target", default = "default_output_target")]
        target_id: String,
    },
    ScreenShake(ScreenShake),
    NextScene,
    Fade(Fade),
    PlaySound(SoundKey),
    PlaySong(SongKey),
    OutputNextLine {
        id:          String,
        #[serde(alias = "target", default = "default_output_target")]
        target_id:   String,
        #[serde(alias = "scroll", default)]
        does_scroll: bool,
    },
    SetVelocity {
        #[serde(default)]
        x: Option<f32>,
        #[serde(default)]
        y: Option<f32>,
    },
    AddVelocity {
        #[serde(default)]
        x: Option<f32>,
        #[serde(default)]
        y: Option<f32>,
    },
    SetVariable(String, VariableValue),
    OpAddVariable(String, VariableValue),
    OpSubVariable(String, VariableValue),
    OpMulVariable(String, VariableValue),
    OpDivVariable(String, VariableValue),
    If(IfAction),
    DeleteEntity,
}

fn default_output_target() -> String {
    String::from("ingame_output_text")
}
