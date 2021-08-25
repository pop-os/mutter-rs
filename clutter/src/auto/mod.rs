// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

mod actor;
pub use self::actor::{Actor, NONE_ACTOR};

#[cfg(any(feature = "v1_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
mod constraint;
#[cfg(any(feature = "v1_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
pub use self::constraint::{Constraint, NONE_CONSTRAINT};

#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
mod container;
#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
pub use self::container::{Container, NONE_CONTAINER};

mod content;
pub use self::content::{Content, NONE_CONTENT};

mod input_device;
pub use self::input_device::{InputDevice};

#[cfg(any(feature = "v0_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_2")))]
mod stage;
#[cfg(any(feature = "v0_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_2")))]
pub use self::stage::{Stage, NONE_STAGE};

mod color;
pub use self::color::Color;

mod enums;
#[cfg(any(feature = "v1_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
pub use self::enums::AnimationMode;
#[cfg(any(feature = "v1_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
pub use self::enums::StaticColor;

#[doc(hidden)]
pub mod traits {
    pub use super::actor::ActorExt;
    #[cfg(any(feature = "v1_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    pub use super::constraint::ConstraintExt;
    #[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    pub use super::container::ContainerExt;
    pub use super::content::ContentExt;
    #[cfg(any(feature = "v0_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_2")))]
    pub use super::stage::StageExt;
}
