use crate::resources::AnimationKey;
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
}
