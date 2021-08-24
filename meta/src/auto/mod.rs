// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

mod background;
pub use self::background::{Background};

mod background_actor;
pub use self::background_actor::{BackgroundActor};

mod background_group;
pub use self::background_group::{BackgroundGroup, NONE_BACKGROUND_GROUP};

mod display;
pub use self::display::{Display};

mod selection;
pub use self::selection::{Selection};

mod selection_source;
pub use self::selection_source::{SelectionSource, NONE_SELECTION_SOURCE};

mod window;
pub use self::window::{Window};

mod workspace;
pub use self::workspace::{Workspace};

mod workspace_manager;
pub use self::workspace_manager::{WorkspaceManager};

mod key_binding;
pub use self::key_binding::KeyBinding;

mod rectangle;
pub use self::rectangle::Rectangle;

mod enums;
pub use self::enums::Cursor;
pub use self::enums::DisplayCorner;
pub use self::enums::DisplayDirection;
pub use self::enums::FrameType;
pub use self::enums::GrabOp;
pub use self::enums::MotionDirection;
pub use self::enums::PadActionType;
pub use self::enums::SelectionType;
pub use self::enums::StackLayer;
pub use self::enums::TabList;
pub use self::enums::WindowClientType;
pub use self::enums::WindowType;

mod flags;
pub use self::flags::KeyBindingFlags;
pub use self::flags::MaximizeFlags;

#[doc(hidden)]
pub mod traits {
    pub use super::selection_source::SelectionSourceExt;
}
