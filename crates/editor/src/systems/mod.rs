use bevy::prelude::*;
use sonolil_render::{system_set::RenderTargetSet, MatchRenderSchedule};

mod _viewport;

pub fn register_systems(app: &mut App) {
    app.add_systems(
        MatchRenderSchedule,
        _viewport::render_target_to_viewport.after(RenderTargetSet),
    );
    //app.add_systems(schedule, systems)
}
