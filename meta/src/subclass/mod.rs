mod plugin;

pub mod prelude {
    pub use super::plugin::PluginImpl;
    pub use glib::subclass::prelude::*;
}
