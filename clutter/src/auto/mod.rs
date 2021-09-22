// Generated by gir (https://github.com/gtk-rs/gir @ 45cd7bc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

mod action;
pub use self::action::{Action, NONE_ACTION};

mod actor;
pub use self::actor::{Actor, NONE_ACTOR};

#[cfg(any(feature = "v1_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
mod actor_meta;
#[cfg(any(feature = "v1_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
pub use self::actor_meta::{ActorMeta, NONE_ACTOR_META};

#[cfg(any(feature = "v1_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
mod align_constraint;
#[cfg(any(feature = "v1_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
pub use self::align_constraint::{AlignConstraint};

mod animatable;
pub use self::animatable::{Animatable, NONE_ANIMATABLE};

#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
mod backend;
#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
pub use self::backend::{Backend};

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
mod canvas;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub use self::canvas::{Canvas, NONE_CANVAS};

#[cfg(any(feature = "v1_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
mod clone;
#[cfg(any(feature = "v1_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
pub use self::clone::{Clone, NONE_CLONE};

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
mod constraint;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
pub use self::constraint::{Constraint, NONE_CONSTRAINT};

#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
mod container;
#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
pub use self::container::{Container, NONE_CONTAINER};

mod content;
pub use self::content::{Content, NONE_CONTENT};

#[cfg(any(feature = "v1_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
mod effect;
#[cfg(any(feature = "v1_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
pub use self::effect::{Effect, NONE_EFFECT};

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
mod image;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub use self::image::{Image, NONE_IMAGE};

mod input_device;
pub use self::input_device::{InputDevice};

mod input_focus;
pub use self::input_focus::{InputFocus, NONE_INPUT_FOCUS};

mod input_method;
pub use self::input_method::{InputMethod, NONE_INPUT_METHOD};

#[cfg(any(feature = "v0_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
mod scriptable;
#[cfg(any(feature = "v0_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
pub use self::scriptable::{Scriptable, NONE_SCRIPTABLE};

#[cfg(any(feature = "v0_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_2")))]
mod stage;
#[cfg(any(feature = "v0_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_2")))]
pub use self::stage::{Stage, NONE_STAGE};

#[cfg(any(feature = "v1_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
mod text;
#[cfg(any(feature = "v1_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
pub use self::text::{Text, NONE_TEXT};

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
mod text_buffer;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub use self::text_buffer::{TextBuffer, NONE_TEXT_BUFFER};

mod color;
pub use self::color::Color;

mod enums;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub use self::enums::ActorAlign;
#[cfg(any(feature = "v1_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
pub use self::enums::AlignAxis;
#[cfg(any(feature = "v1_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
pub use self::enums::AnimationMode;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub use self::enums::ContentGravity;
#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
pub use self::enums::EventType;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
pub use self::enums::Orientation;
#[cfg(any(feature = "v0_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
pub use self::enums::RequestMode;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub use self::enums::ScalingFilter;
#[cfg(any(feature = "v1_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
pub use self::enums::StaticColor;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
pub use self::enums::TextDirection;

mod flags;
pub use self::flags::ActorFlags;
#[cfg(any(feature = "v0_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
pub use self::flags::EventFlags;
#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
pub use self::flags::ModifierType;

pub mod functions;

#[doc(hidden)]
pub mod traits {
    pub use super::actor::ActorExt;
    #[cfg(any(feature = "v1_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    pub use super::actor_meta::ActorMetaExt;
    pub use super::animatable::AnimatableExt;
    #[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub use super::canvas::CanvasExt;
    #[cfg(any(feature = "v1_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    pub use super::clone::CloneExt;
    #[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    pub use super::constraint::ConstraintExt;
    #[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    pub use super::container::ContainerExt;
    pub use super::content::ContentExt;
    #[cfg(any(feature = "v1_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
    pub use super::effect::EffectExt;
    #[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub use super::image::ImageExt;
    pub use super::input_focus::InputFocusExt;
    pub use super::input_method::InputMethodExt;
    #[cfg(any(feature = "v0_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    pub use super::scriptable::ScriptableExt;
    #[cfg(any(feature = "v0_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_2")))]
    pub use super::stage::StageExt;
    #[cfg(any(feature = "v1_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    pub use super::text::TextExt;
    #[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub use super::text_buffer::TextBufferExt;
}
