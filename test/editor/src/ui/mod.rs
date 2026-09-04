use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod docks;
pub mod view;

pub struct EditorPlugins;

impl PluginGroup for EditorPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(docks::DockPlugin)
            .add(view::ViewerPlugin)
    }
}
