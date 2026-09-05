use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod test;
pub mod viewport;

pub struct EditorPlugins;

impl PluginGroup for EditorPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(test::DockPlugin)
            .add(viewport::ViewerPlugin)
    }
}
