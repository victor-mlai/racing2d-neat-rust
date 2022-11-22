pub mod components;
pub mod resources;

pub struct BoardPlugin;

use bevy::log;
use bevy::prelude::*;
use components::*;
use resources::tile_map::TileMap;
use resources::BoardOptions;
use resources::BoardPosition;
use resources::TileSize;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        log::info!("Created board plugin");

        //#[cfg(feature = "debug")]
        //{
        //    app.register_inspectable::<components::Coordinates>();
        //    app.register_inspectable::<components::Bomb>();
        //    app.register_inspectable::<components::BombNeighbor>();
        //    app.register_inspectable::<components::Uncover>();
        //}
    }
}

impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Res<WindowDescriptor>,
        asset_server: Res<AssetServer>, // AssetServer allows loading files from the assets folder
    ) {
        let options = match board_options {
            Some(ops) => ops.clone(),
            None => Default::default(),
        };

        //let font = asset_server.load("fonts/pixeled.ttf");
        //let bomb_image = asset_server.load("sprites/bomb.png");

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => {
                Self::adaptative_tile_size(window, (min, max), options.map_size)
            }
        };

        let board_size = Vec2::new(
            options.map_size.1 as f32 * tile_size,
            options.map_size.0 as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        // We define the board anchor position (bottom left)
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        let mut tile_map = TileMap::new(options.map_size.0, options.map_size.1);
        tile_map.add_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                for (y, line) in tile_map.iter().enumerate() {
                    for (x, tile) in line.iter().enumerate() {
                        parent
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::GRAY,
                                    custom_size: Some(Vec2::splat(
                                        tile_size - options.tile_padding as f32,
                                    )),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(
                                    (x as f32 * tile_size) + (tile_size / 2.),
                                    (y as f32 * tile_size) + (tile_size / 2.),
                                    1.,
                                ),
                                ..Default::default()
                            })
                            .insert(Name::new(format!("Tile ({}, {})", x, y)))
                            // We add the `Coordinates` component to our tile entity
                            .insert(Coordinates {
                                x: x as u16,
                                y: y as u16,
                            });
                    }
                }
            });
    }

    /// Computes a tile size that matches the window according to the tile map size
    fn adaptative_tile_size(
        window: Res<WindowDescriptor>,
        (min, max): (f32, f32),      //tile size
        (width, height): (u16, u16), //tile map size
    ) -> f32 {
        let max_width = window.width / width as f32;
        let max_heigth = window.height / height as f32;
        max_width.min(max_heigth).clamp(min, max)
    }
}
