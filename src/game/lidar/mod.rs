mod dots;

use crate::game::lidar::dots::LidarDotsPlugin;
use bevy::prelude::*;

pub(super) struct LidarPlugin;

impl Plugin for LidarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LidarDotsPlugin);
    }
}
