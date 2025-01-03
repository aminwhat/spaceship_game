mod debug;
mod movement;
mod spaceship;

use bevy::prelude::*;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .add_plugins(DebugPlugin)
        //* User Configured Plugins */
        .add_plugins(SpaceshipPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(MovementPlugin)
        .run();
}
