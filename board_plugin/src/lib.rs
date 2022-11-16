pub mod components;
pub mod resources;

pub struct BoardPlugin;

use bevy::log;
use bevy::prelude::*;
use resources::tile_map::TileMap;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        log::info!("Created board plugin");
    }
}

impl BoardPlugin {
    pub fn create_board() {
        let mut tile_map = TileMap::new(20, 20);
        tile_map.add_bombs(40);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }
}
